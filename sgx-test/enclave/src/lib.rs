#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"), feature(rustc_private))]

#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

use std::prelude::v1::*;

extern crate sgx_types;
use sgx_types::*;

use eigen_crypto::sign::ecdsa::KeyPair;

#[no_mangle]
pub extern "C" fn ecall_run_tests() -> sgx_status_t {
    test_eigen_crypto();

    println!("passed all tests");
    sgx_status_t::SGX_SUCCESS
}

fn test_eigen_crypto() {
    test_ecies();
}

fn test_ecies() {
    /*
    let d = "29079635126530934056640915735344231956621504557963207107451663058887647996601";
    let seed_bytes = num_bigint::BigInt::from_str(&d).unwrap().to_bytes_be();
    let alg = &crate::sign::ecdsa::ECDSA_P256_SHA256_ASN1_SIGNING;
    let seed = untrusted::Input::from(&seed_bytes.1);
    let private_key = crate::sign::ecdsa::EcdsaKeyPair::from_seed_unchecked(alg, seed);
    */
    use rand::Rng;

    let mut r = vec![0u8; 32];
    rand::thread_rng().fill(&mut r[..]);
    let private_key = eigen_crypto::sign::ecdsa::EcdsaKeyPair::from_seed_unchecked(
        &eigen_crypto::sign::ecdsa::ECDSA_P256_SHA256_ASN1_SIGNING,
        untrusted::Input::from(&r),
    );

    assert_eq!(private_key.is_ok(), true);
    let private_key = private_key.unwrap();
    let msg = "hello, come on, go get it 你好!";
    let s1 = vec![];
    let s2 = vec![];

    let alg = &eigen_crypto::sign::ecdsa::ECDSA_P256_SHA256_ASN1;
    let public_key = eigen_crypto::sign::ecdsa::UnparsedPublicKey::new(alg, private_key.public_key());

    let cipher = eigen_crypto::ec::suite_b::ecies::encrypt(&public_key, &s1, &s2, msg.as_bytes());
    assert_eq!(cipher.is_ok(), true);
    let cipher = cipher.unwrap();
    let plain = eigen_crypto::ec::suite_b::ecies::decrypt(&private_key, &cipher, &s1, &s2);

    assert_eq!(plain.is_ok(), true);
    assert_eq!(msg.as_bytes().to_vec(), (plain.unwrap()));
}
