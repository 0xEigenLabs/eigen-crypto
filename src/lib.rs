#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
extern crate sgx_libc as libc;

#[cfg(any(feature = "mesalock_sgx", target_env = "sgx"))]
extern crate ring_sgx as ring;
#[cfg(not(any(feature = "mesalock_sgx", target_env = "sgx")))]
extern crate ring;

#[cfg(any(feature = "mesalock_sgx", target_env = "sgx"))]
extern crate rand_sgx as rand;
#[cfg(not(any(feature = "mesalock_sgx", target_env = "sgx")))]
extern crate rand;

#[cfg(any(feature = "mesalock_sgx", target_env = "sgx"))]
extern crate num_bigint_sgx as num_bigint;
#[cfg(not(any(feature = "mesalock_sgx", target_env = "sgx")))]
extern crate num_bigint;

#[cfg(any(feature = "mesalock_sgx", target_env = "sgx"))]
extern crate rust_base58_sgx as rust_base58;
#[cfg(not(any(feature = "mesalock_sgx", target_env = "sgx")))]
extern crate rust_base58;

#[cfg(any(feature = "mesalock_sgx", target_env = "sgx"))]
extern crate rust_crypto_sgx as crypto;
#[cfg(not(any(feature = "mesalock_sgx", target_env = "sgx")))]
extern crate crypto;

#[cfg(any(feature = "mesalock_sgx", target_env = "sgx"))]
extern crate serde_sgx as serde;
#[cfg(not(any(feature = "mesalock_sgx", target_env = "sgx")))]
extern crate serde;

#[cfg(any(feature = "mesalock_sgx", target_env = "sgx"))]
extern crate serde_derive_sgx as serde_derive;
#[cfg(not(any(feature = "mesalock_sgx", target_env = "sgx")))]
extern crate serde_derive;

#[cfg(any(feature = "mesalock_sgx", target_env = "sgx"))]
extern crate num_traits_sgx as num_traits;
#[cfg(not(any(feature = "mesalock_sgx", target_env = "sgx")))]
extern crate num_traits;

#[cfg(any(feature = "mesalock_sgx", target_env = "sgx"))]
extern crate num_integer_sgx as num_integer;
#[cfg(not(any(feature = "mesalock_sgx", target_env = "sgx")))]
extern crate num_integer;

#[cfg(any(feature = "mesalock_sgx", target_env = "sgx"))]
extern crate untrusted_sgx as untrusted;
#[cfg(not(any(feature = "mesalock_sgx", target_env = "sgx")))]
extern crate untrusted;

#[cfg(any(feature = "mesalock_sgx", target_env = "sgx"))]
extern crate bytes_sgx as bytes;
#[cfg(not(any(feature = "mesalock_sgx", target_env = "sgx")))]
extern crate bytes;

#[cfg(any(feature = "mesalock_sgx", target_env = "sgx"))]
extern crate regex_sgx as regex;
#[cfg(not(any(feature = "mesalock_sgx", target_env = "sgx")))]
extern crate regex;

#[macro_use]
extern crate lazy_static;


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
