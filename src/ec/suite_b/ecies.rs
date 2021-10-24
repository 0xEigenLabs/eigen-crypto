use crate::arithmetic::montgomery::R;
use crate::sign::ecdsa::EcdsaKeyPair;
use crate::sign::ecdsa::KeyPair;
use std::convert::TryInto;
use std::prelude::v1::*;

use crate::errors::Result;
use crate::errors::*;
use crate::ring::aead::BoundKey;
use crate::ring::digest;
use bytes::{BufMut, BytesMut};
use rand::Rng;

use crypto::aead::{AeadDecryptor, AeadEncryptor};
use crypto::aes::KeySize;
use crypto::aes_gcm::AesGcm;

const INITIAL_SALT: [u8; 20] = [
    0xc3, 0xee, 0xf7, 0x12, 0xc7, 0x2e, 0xbb, 0x5a, 0x11, 0xa7, 0xd2, 0x43, 0x2b, 0xb4, 0x63, 0x65,
    0xbe, 0xf9, 0xf5, 0x02,
];
const LABEL: &[u8] = b"eigen-crypto";

#[allow(non_snake_case)]
pub fn encrypt<B: AsRef<[u8]>>(
    public_key: &crate::sign::ecdsa::UnparsedPublicKey<B>,
    s1: &[u8],
    s2: &[u8],
    msg: &[u8],
) -> Result<Vec<u8>> {
    let public_key_ops = &super::ops::p256::PUBLIC_KEY_OPS;
    let private_key_ops = &super::ops::p256::PRIVATE_KEY_OPS;
    let common_ops = &super::ops::p256::COMMON_OPS;

    // generate random r
    let mut r = vec![0u8; 32];
    rand::thread_rng().fill(&mut r[..]);
    let sk = crate::sign::ecdsa::EcdsaKeyPair::from_seed_unchecked(
        &crate::sign::ecdsa::ECDSA_P256_SHA256_ASN1_SIGNING,
        untrusted::Input::from(&r),
    )?;

    let R = sk.public_key();
    // derive shared secret: S = P_x, where P = r * K_b
    let K_b = super::public_key::parse_uncompressed_point(
        &public_key_ops,
        untrusted::Input::from(public_key.as_ref()),
    )?;

    let d = super::scalar_parse_big_endian_variable(
        common_ops,
        crate::limb::AllowZero::No,
        untrusted::Input::from(&sk.seed_as_bytes()),
    )?;
    // P = r * K_b
    let P = private_key_ops.point_mul(&d, &K_b);

    let mut actual_result = vec![4u8; 1 + (2 * (common_ops.num_limbs * crate::limb::LIMB_BYTES))];
    let (x, y) = actual_result[1..].split_at_mut(common_ops.num_limbs * crate::limb::LIMB_BYTES);
    super::private_key::big_endian_affine_from_jacobian(private_key_ops, Some(x), Some(y), &P)?;
    // k_e, k_m = KDF(S || S_1)
    let (k_e, k_m) = deriveKeys(&x, &s1)?;

    let c = aes_encrypt_less_safe(&k_e, msg)?;
    let d = message_tag(&k_m, &c, s2)?;

    //R || c || d
    let mut res = vec![0u8; 65];
    res.copy_from_slice(R.as_ref());
    res.extend_from_slice(&c);
    res.extend_from_slice(&d);
    Ok(res)
}

#[allow(non_snake_case)]
pub fn decrypt(sk: &EcdsaKeyPair, c: &[u8], s1: &[u8], s2: &[u8]) -> Result<Vec<u8>> {
    let public_key_ops = &super::ops::p256::PUBLIC_KEY_OPS;
    let private_key_ops = &super::ops::p256::PRIVATE_KEY_OPS;
    let common_ops = &super::ops::p256::COMMON_OPS;
    let R = &c[0..65];
    let R =
        super::public_key::parse_uncompressed_point(&public_key_ops, untrusted::Input::from(R))?;
    // S = P_x, P = (P_x, P_y) = k_B * R
    let k_B = super::scalar_parse_big_endian_variable(
        common_ops,
        crate::limb::AllowZero::No,
        // seed_as_bytes returns the real key !!!!!
        untrusted::Input::from(&sk.seed_as_bytes()),
    )?;

    let P = private_key_ops.point_mul(&k_B, &R);

    let mut actual_result = vec![4u8; 1 + (2 * (common_ops.num_limbs * crate::limb::LIMB_BYTES))];
    let (x, y) = actual_result[1..].split_at_mut(common_ops.num_limbs * crate::limb::LIMB_BYTES);
    super::private_key::big_endian_affine_from_jacobian(private_key_ops, Some(x), Some(y), &P)?;

    // k_e, k_m = KDF(S || S_1)
    let (k_e, k_m) = deriveKeys(&x, &s1)?;

    let cc = &c[65..(c.len() - 32)];
    let d = message_tag(&k_m, cc, s2)?;
    let dd = &c[(c.len() - 32)..];
    // compare k_m and  d
    if d != dd {
        return Err(Error::from(ErrorKind::CryptoError));
    }

    let m = aes_decrypt_less_safe(&k_e, cc)?;
    return Ok(m);
}

//TODO: https://github.com/ethereum/go-ethereum/blob/master/crypto/ecies/ecies.go#L146
fn deriveKeys(key: &[u8], s1: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
    let mut concat_key = vec![];
    concat_key.extend_from_slice(&key);
    concat_key.extend_from_slice(&s1);
    let secret = digest::digest(&digest::SHA512, &concat_key)
        .as_ref()
        .to_vec();
    let (k_e, k_m) = secret.split_at(secret.len() / 2);
    Ok((k_e.to_vec(), k_m.to_vec()))
}

fn message_tag(k_m: &[u8], c: &[u8], s2: &[u8]) -> Result<Vec<u8>> {
    let mut msg = vec![];
    msg.extend_from_slice(c);
    msg.extend_from_slice(s2);
    let s_key = ring::hmac::Key::new(ring::hmac::HMAC_SHA256, &k_m);
    let signature = ring::hmac::sign(&s_key, &msg);
    Ok(signature.as_ref().to_vec())
}

// we set IV equal to nonce, less safer compared to aes_encrypt
pub fn aes_encrypt_less_safe(key: &[u8], msg: &[u8]) -> Result<Vec<u8>> {
    let add = [0u8; 0];
    let mut nonce = [0u8; 12];
    rand::thread_rng().fill(&mut nonce[..]);
    let mut cipher = AesGcm::new(
        KeySize::KeySize256,
        key,    //32
        &nonce, //12
        &add,
    ); //0
    let mut output = vec![0u8; msg.len()];

    let mut tag = [0u8; 16];

    cipher.encrypt(msg, &mut output, &mut tag);

    let mut result = BytesMut::with_capacity(output.len() + nonce.len() + tag.len());
    result.put_slice(&nonce);
    result.put_slice(&tag);
    result.put_slice(&output);
    Ok(result.to_vec())
}

pub fn aes_decrypt_less_safe(key: &[u8], c: &[u8]) -> Result<Vec<u8>> {
    if c.len() <= 28 {
        // TODO: the length (now 28) should be configurable
        // Not enough long, obviously wrong
        return Err(Error::from(ErrorKind::CryptoError));
    }

    let add = [0u8; 0];
    let nonce = &c[0..12];
    let tag = &c[12..28];
    let cipher = &c[28..];

    let mut decipher = AesGcm::new(KeySize::KeySize256, key, nonce, &add);
    let mut output = vec![0u8; cipher.len()];
    decipher.decrypt(cipher, &mut output, tag);
    Ok(output)
}

fn aes_encrypt(key: &[u8], msg: &[u8]) -> Result<Vec<u8>> {
    let ubk = ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, key)
        .map_err(|_| Error::from(ErrorKind::CryptoError))?;
    let mut r = [0u8; ring::aead::NONCE_LEN];
    rand::thread_rng().fill(&mut r[..]);
    let nonce = RingAeadNonceSequence::new(r);
    let mut sealing_key = ring::aead::SealingKey::new(ubk, nonce);
    let in_out_len = msg.len() + ring::aead::AES_256_GCM.tag_len();
    let mut in_out = BytesMut::with_capacity(in_out_len);
    in_out.put_slice(msg);
    let aad = ring::aead::Aad::empty();
    sealing_key
        .seal_in_place_append_tag(aad, &mut in_out)
        .map_err(|_| Error::from(ErrorKind::CryptoError))?;
    let mut out = BytesMut::with_capacity(ring::aead::NONCE_LEN + in_out_len);
    out.put_slice(nonce.as_ref());
    out.put_slice(&in_out[..in_out_len]);
    Ok(out.to_vec())
}

fn aes_decrypt(key: &[u8], c: &[u8]) -> Result<Vec<u8>> {
    let ubk = ring::aead::UnboundKey::new(&ring::aead::AES_256_GCM, key).unwrap();
      //  .map_err(|_| Error::from(ErrorKind::CryptoError))?;
    let nonce = RingAeadNonceSequence::new((c[..ring::aead::NONCE_LEN]).try_into().unwrap());
    let mut opening_key = ring::aead::OpeningKey::new(ubk, nonce);
    let mut in_out = BytesMut::with_capacity(c.len() - ring::aead::NONCE_LEN);
    in_out.put_slice(&c[ring::aead::NONCE_LEN..c.len()]);

    let aad = ring::aead::Aad::empty();
    let m = opening_key
        .open_in_place(aad, &mut in_out)
        .unwrap();
        //.map_err(|_| Error::from(ErrorKind::CryptoError))?;
    Ok(m.to_vec())
}

impl RingAeadNonceSequence {
    fn new(n: [u8; ring::aead::NONCE_LEN]) -> RingAeadNonceSequence {
        RingAeadNonceSequence { nonce: n }
    }
}

impl AsRef<[u8]> for RingAeadNonceSequence {
    fn as_ref(&self) -> &[u8] {
        &self.nonce
    }
}

impl ring::aead::NonceSequence for RingAeadNonceSequence {
    fn advance(&mut self) -> std::result::Result<ring::aead::Nonce, ring::error::Unspecified> {
        Ok(ring::aead::Nonce::assume_unique_for_key(self.nonce))
    }
}

#[derive(Copy, Clone)]
pub struct RingAeadNonceSequence {
    nonce: [u8; ring::aead::NONCE_LEN],
}

#[cfg(test)]
mod tests {
    use super::*;
    pub use crate::sign::ecdsa::KeyPair;
    use rand::Rng;
    use std::str::FromStr;
    #[test]
    fn test_aes() {
        let mut key = vec![0u8; 32];
        rand::thread_rng().fill(&mut key[..]);

        let msg = "Eigen, hello, world";
        let c = aes_encrypt(&key, msg.as_bytes());
        assert_eq!(c.is_ok(), true);
        let c = c.unwrap();

        let p = aes_decrypt(&key, &c);
        assert_eq!(p.is_ok(), true);
        let p = p.unwrap();
        assert_eq!(p, msg.as_bytes().to_vec());
    }

    #[test]
    fn test_aes_less_safe() {
        let mut key = vec![0u8; 32];
        rand::thread_rng().fill(&mut key[..]);

        let msg = "Eigen, hello, world";
        let c = aes_encrypt_less_safe(&key, msg.as_bytes());
        assert_eq!(c.is_ok(), true);
        let c = c.unwrap();

        let p = aes_decrypt_less_safe(&key, &c);
        assert_eq!(p.is_ok(), true);
        let p = p.unwrap();
        assert_eq!(p, msg.as_bytes().to_vec());

        let hexCipher = hex::decode("18cc968dff2f8a4fa87f2d964a3e0aacd359659722e52706f8d5d29d8c141025f897509177cdae93a905b0b6a7a4e9cd2f611e47a8eb68da839d96ca").unwrap();
        let key = "01234567890123456789123456123456";
        let p = aes_decrypt_less_safe(key.as_bytes(), &hexCipher);
        println!("{:?}", p);

        let msg = "Hello, Eigen, Privacy Computing!";
        assert_eq!(p.is_ok(), true);
        let p = p.unwrap();
        println!("{:?}", p);
        println!("{:?}", msg.as_bytes().to_vec());
        assert_eq!(p, msg.as_bytes().to_vec());
    }

    #[test]
    fn test_ecies() {
        let mut r = vec![0u8; 32];
        rand::thread_rng().fill(&mut r[..]);
        let private_key = crate::sign::ecdsa::EcdsaKeyPair::from_seed_unchecked(
            &crate::sign::ecdsa::ECDSA_P256_SHA256_ASN1_SIGNING,
            untrusted::Input::from(&r),
        );

        assert_eq!(private_key.is_ok(), true);
        let private_key = private_key.unwrap();
        println!("KeyPair {:?}", private_key);
        let msg = "Hello, Eigen, Privacy Computing!";
        let s1 = vec![];
        let s2 = vec![];

        let alg = &crate::sign::ecdsa::ECDSA_P256_SHA256_ASN1;
        let public_key = crate::sign::ecdsa::UnparsedPublicKey::new(alg, private_key.public_key());

        let cipher = encrypt(&public_key, &s1, &s2, msg.as_bytes());
        assert_eq!(cipher.is_ok(), true);
        let cipher = cipher.unwrap();
        let plain = decrypt(&private_key, &cipher, &s1, &s2);

        assert_eq!(plain.is_ok(), true);
        assert_eq!(msg.as_bytes().to_vec(), (plain.unwrap()));
    }

    #[test]
    fn test_ecies_with_js() {
        let entropy =
            "29079635126530934056640915735344231956621504557963207107451663058887647996601";
        let entropy_bytes = num_bigint::BigInt::from_str(&entropy)
            .unwrap()
            .to_bytes_be();
        let alg = &crate::sign::ecdsa::ECDSA_P256_SHA256_ASN1_SIGNING;
        let seed = untrusted::Input::from(&entropy_bytes.1);
        let private_key = crate::sign::ecdsa::EcdsaKeyPair::from_seed_unchecked(alg, seed);

        assert_eq!(private_key.is_ok(), true);
        let private_key = private_key.unwrap();
        println!("KeyPair {:?}", private_key);
        let msg = "Hello, Eigen, Privacy Computing!";
        let s1 = vec![];
        let s2 = vec![];

        let alg = &crate::sign::ecdsa::ECDSA_P256_SHA256_ASN1;
        let public_key = crate::sign::ecdsa::UnparsedPublicKey::new(alg, private_key.public_key());

        let cipher = encrypt(&public_key, &s1, &s2, msg.as_bytes());
        assert_eq!(cipher.is_ok(), true);
        let cipher = hex::decode(
            "04de80176a235a70c45e3511f902527f3c6305ceda8942ef95af1e786552d8c250b44164e8bf79ff35c353a6c4772d1f5287011b8ffe9c02a725623798a57d7b78bde42d6375a0df5d025df51145317f7b1f27f4c8a66ea185f5518ae3258f8c5fc5d9f74fbd4a41b1a44c03040f6d8b7d6e2f01f3cd327626d22a073be5e8d9fd7651b3c26e416cf4d945bfeb6a0c62b5acd10413c3167d118d7f5b11").unwrap();
        let plain = decrypt(&private_key, &cipher, &s1, &s2);

        assert_eq!(plain.is_ok(), true);
        assert_eq!(msg.as_bytes().to_vec(), (plain.unwrap()));
    }
}
