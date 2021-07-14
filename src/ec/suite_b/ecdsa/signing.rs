// Copyright 2015-2016 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

//! ECDSA Signatures using the P-256 and P-384 curves.
use std::prelude::v1::*;

use super::digest_scalar::digest_scalar;
use crate::{
    arithmetic::montgomery::*,
    ec::{
        self,
        suite_b::{ops::*, private_key},
    },
    errors::{Error, ErrorKind, Result},
    io::der,
    limb,
};
use ring::digest;

/// An ECDSA signing algorithm.
pub struct EcdsaSigningAlgorithm {
    curve: &'static ec::Curve,
    private_scalar_ops: &'static PrivateScalarOps,
    private_key_ops: &'static PrivateKeyOps,
    digest_alg: &'static digest::Algorithm,
    format_rs: fn(ops: &'static ScalarOps, r: &Scalar, s: &Scalar, out: &mut [u8]) -> usize,
    id: AlgorithmID,
}

#[derive(Debug, Eq, PartialEq)]
enum AlgorithmID {
    ECDSA_P256_SHA256_ASN1_SIGNING,
}

derive_debug_via_id!(EcdsaSigningAlgorithm);

impl PartialEq for EcdsaSigningAlgorithm {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for EcdsaSigningAlgorithm {}

/// An ECDSA key pair, used for signing.
pub struct EcdsaKeyPair {
    d: Scalar<R>,
    alg: &'static EcdsaSigningAlgorithm,
    public_key: PublicKey,
    seed: Vec<u8>, // For backup
}

derive_debug_via_field!(EcdsaKeyPair, stringify!(EcdsaKeyPair), public_key);

impl EcdsaKeyPair {
    /// Generates a new key pair and returns the key pair serialized as a
    /// PKCS#8 document.
    ///
    /// The PKCS#8 document will be a v1 `OneAsymmetricKey` with the public key
    /// included in the `ECPrivateKey` structure, as described in
    /// [RFC 5958 Section 2] and [RFC 5915]. The `ECPrivateKey` structure will
    /// not have a `parameters` field so the generated key is compatible with
    /// PKCS#11.
    ///
    /// [RFC 5915]: https://tools.ietf.org/html/rfc5915
    /// [RFC 5958 Section 2]: https://tools.ietf.org/html/rfc5958#section-2
    /*
    pub fn generate_pkcs8(
        alg: &'static EcdsaSigningAlgorithm,
        rng: &dyn rand::SecureRandom,
    ) -> Result<pkcs8::Document> {
        let private_key = ec::Seed::generate(alg.curve, rng, cpu::features())?;
        let public_key = private_key.compute_public_key()?;
        Ok(pkcs8::wrap_key(
            &alg.pkcs8_template,
            private_key.bytes_less_safe(),
            public_key.as_ref(),
        ))
    }
    */

    /// Constructs an ECDSA key pair by parsing an unencrypted PKCS#8 v1
    /// id-ecPublicKey `ECPrivateKey` key.
    ///
    /// The input must be in PKCS#8 v1 format. It must contain the public key in
    /// the `ECPrivateKey` structure; `from_pkcs8()` will verify that the public
    /// key and the private key are consistent with each other. The algorithm
    /// identifier must identify the curve by name; it must not use an
    /// "explicit" encoding of the curve. The `parameters` field of the
    /// `ECPrivateKey`, if present, must be the same named curve that is in the
    /// algorithm identifier in the PKCS#8 header.
    /*
    pub fn from_pkcs8(
        alg: &'static EcdsaSigningAlgorithm,
        pkcs8: &[u8],
    ) -> Result<Self, error::KeyRejected> {
        let key_pair = ec::suite_b::key_pair_from_pkcs8(
            alg.curve,
            alg.pkcs8_template,
            untrusted::Input::from(pkcs8),
            cpu::features(),
        )?;
        Ok(Self::new(alg, key_pair))
    }
    */

    pub fn from_seed_unchecked(
        alg: &'static EcdsaSigningAlgorithm,
        salt: untrusted::Input,
    ) -> Result<Self> {
        let seed = ec::Seed::from_bytes(alg.curve, salt)?;
        let key_pair = ec::KeyPair::derive(seed)?;
        Ok(Self::new(alg, key_pair))
    }

    fn new(alg: &'static EcdsaSigningAlgorithm, key_pair: ec::KeyPair) -> Self {
        let (seed, public_key) = key_pair.split();
        let d = private_key::private_key_as_scalar(alg.private_key_ops, &seed);
        let d = alg
            .private_scalar_ops
            .scalar_ops
            .scalar_product(&d, &alg.private_scalar_ops.oneRR_mod_n);

        Self {
            seed: seed.bytes_less_safe().to_vec(),
            d,
            alg,
            public_key: PublicKey(public_key),
        }
    }

    pub fn seed_as_bytes(&self) -> Vec<u8> {
        self.seed.to_owned().into()
    }

    /// Returns the signature of the `message` using a random nonce
    /// generated by `rng`.
    pub fn sign(&self, message: &[u8]) -> Result<crate::sign::ecdsa::Signature> {
        // Step 4 (out of order).
        let h = digest::digest(self.alg.digest_alg, message);
        self.sign_(h)
    }

    /// Returns the signature of message digest `h` using a "random" nonce
    /// generated by `rng`.
    fn sign_(&self, h: digest::Digest) -> Result<crate::sign::ecdsa::Signature> {
        // NSA Suite B Implementer's Guide to ECDSA Section 3.4.1: ECDSA
        // Signature Generation.

        // NSA Guide Prerequisites:
        //
        //     Prior to generating an ECDSA signature, the signatory shall
        //     obtain:
        //
        //     1. an authentic copy of the domain parameters,
        //     2. a digital signature key pair (d,Q), either generated by a
        //        method from Appendix A.1, or obtained from a trusted third
        //        party,
        //     3. assurance of the validity of the public key Q (see Appendix
        //        A.3), and
        //     4. assurance that he/she/it actually possesses the associated
        //        private key d (see [SP800-89] Section 6).
        //
        // The domain parameters are hard-coded into the source code.
        // `EcdsaKeyPair::generate_pkcs8()` can be used to meet the second
        // requirement; otherwise, it is up to the user to ensure the key pair
        // was obtained from a trusted private key. The constructors for
        // `EcdsaKeyPair` ensure that #3 and #4 are met subject to the caveats
        // in SP800-89 Section 6.

        let ops = self.alg.private_scalar_ops;
        let scalar_ops = ops.scalar_ops;
        let cops = scalar_ops.common;
        let private_key_ops = self.alg.private_key_ops;

        for _ in 0..100 {
            // XXX: iteration conut?
            // Step 1.
            let k = private_key::random_scalar(self.alg.private_key_ops)?;
            let k_inv = scalar_ops.scalar_inv_to_mont(&k);

            // Step 2.
            let r = private_key_ops.point_mul_base(&k);

            // Step 3.
            let r = {
                let (x, _) = private_key::affine_from_jacobian(private_key_ops, &r)?;
                let x = cops.elem_unencoded(&x);
                elem_reduced_to_scalar(cops, &x)
            };
            if cops.is_zero(&r) {
                continue;
            }

            // Step 4 is done by the caller.

            // Step 5.
            let e = digest_scalar(scalar_ops, h);

            // Step 6.
            let s = {
                let dr = scalar_ops.scalar_product(&self.d, &r);
                let e_plus_dr = scalar_sum(cops, &e, &dr);
                scalar_ops.scalar_product(&k_inv, &e_plus_dr)
            };
            if cops.is_zero(&s) {
                continue;
            }

            // Step 7 with encoding.
            return Ok(crate::sign::ecdsa::Signature::new(|sig_bytes| {
                (self.alg.format_rs)(scalar_ops, &r, &s, sig_bytes)
            }));
        }

        Err(Error::from(ErrorKind::CryptoError))
    }
}

impl crate::sign::ecdsa::KeyPair for EcdsaKeyPair {
    type PublicKey = PublicKey;

    fn public_key(&self) -> &Self::PublicKey {
        &self.public_key
    }
}

#[derive(Clone, Copy)]
pub struct PublicKey(ec::PublicKey);

derive_debug_self_as_ref_hex_bytes!(PublicKey);

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

/// Signing of ASN.1 DER-encoded ECDSA signatures using the P-256 curve and
/// SHA-256.
///
/// See "`ECDSA_*_ASN1` Details" in `ring::signature`'s module-level
/// documentation for more details.
pub static ECDSA_P256_SHA256_ASN1_SIGNING: EcdsaSigningAlgorithm = EcdsaSigningAlgorithm {
    curve: &ec::suite_b::curve::P256,
    private_scalar_ops: &p256::PRIVATE_SCALAR_OPS,
    private_key_ops: &p256::PRIVATE_KEY_OPS,
    digest_alg: &digest::SHA256,
    format_rs: format_rs_asn1,
    id: AlgorithmID::ECDSA_P256_SHA256_ASN1_SIGNING,
};

fn format_rs_asn1(ops: &'static ScalarOps, r: &Scalar, s: &Scalar, out: &mut [u8]) -> usize {
    // This assumes `a` is not zero since neither `r` or `s` is allowed to be
    // zero.
    fn format_integer_tlv(ops: &ScalarOps, a: &Scalar, out: &mut [u8]) -> usize {
        let mut fixed = [0u8; ec::SCALAR_MAX_BYTES + 1];
        let fixed = &mut fixed[..(ops.scalar_bytes_len() + 1)];
        limb::big_endian_from_limbs(&a.limbs[..ops.common.num_limbs], &mut fixed[1..]);

        // Since `a_fixed_out` is an extra byte long, it is guaranteed to start
        // with a zero.
        debug_assert_eq!(fixed[0], 0);

        // There must be at least one non-zero byte since `a` isn't zero.
        let first_index = fixed.iter().position(|b| *b != 0).unwrap();

        // If the first byte has its high bit set, it needs to be prefixed with 0x00.
        let first_index = if fixed[first_index] & 0x80 != 0 {
            first_index - 1
        } else {
            first_index
        };
        let value = &fixed[first_index..];

        out[0] = der::Tag::Integer as u8;

        // Lengths less than 128 are encoded in one byte.
        assert!(value.len() < 128);
        out[1] = value.len() as u8;

        out[2..][..value.len()].copy_from_slice(&value);

        2 + value.len()
    }

    out[0] = der::Tag::Sequence as u8;
    let r_tlv_len = format_integer_tlv(ops, r, &mut out[2..]);
    let s_tlv_len = format_integer_tlv(ops, s, &mut out[2..][r_tlv_len..]);

    // Lengths less than 128 are encoded in one byte.
    let value_len = r_tlv_len + s_tlv_len;
    assert!(value_len < 128);
    out[1] = value_len as u8;

    2 + value_len
}
