#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"), feature(rustc_private))]

#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;
// extern crate sgx_libc as libc;

#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
extern crate ring_sgx as ring;
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
extern crate rand_sgx as rand;
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
extern crate num_bigint_sgx as num_bigint;
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
extern crate rust_base58_sgx as rust_base58;
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
extern crate rust_crypto_sgx as rust_crypto;
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
extern crate serde_json_sgx as serde_json;
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
extern crate sgx_serde_derive as serde_derive;
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
extern crate serde_sgx as serde;
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
extern crate num_traits_sgx as num_traits;
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
extern crate num_integer_sgx as num_integer;
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
extern crate untrusted_sgx as untrusted;
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
extern crate bytes_sgx as bytes;
//#[cfg(any(feature = "mesalock_sgx", target_env = "sgx"))]
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
extern crate regex_sgx as regex;

#[macro_use]
extern crate lazy_static;

extern crate rust_base58;
extern crate num_bigint;
extern crate num_traits;
extern crate regex;
extern crate ring;

pub mod account;
pub mod bits;
pub mod c;
pub mod hash;
pub mod hdwallet;
pub mod limb;
pub mod sign;

#[macro_use]
mod debug;

#[macro_use]
pub mod arithmetic;

#[macro_use]
pub mod bssl;

pub mod ec;
pub mod errors;
pub mod io;

#[macro_use]
pub mod test;
