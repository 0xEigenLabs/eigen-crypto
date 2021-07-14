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
