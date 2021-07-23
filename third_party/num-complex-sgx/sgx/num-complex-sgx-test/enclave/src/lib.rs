// Copyright (C) 2017-2018 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(target_env = "sgx")]
extern crate core;

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_tunittest;

use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::slice;
use std::panic;
use sgx_tunittest::*;

extern crate num_complex;
extern crate num_traits as traits;
extern crate rand;

mod tests;
mod cast;
mod crand;

#[no_mangle]
pub extern "C" fn say_something(some_string: *const u8, some_len: usize) -> sgx_status_t {

    let str_slice = unsafe { slice::from_raw_parts(some_string, some_len) };
    let _ = io::stdout().write(str_slice);

    // A sample &'static string
    let rust_raw_string = "This is a in-Enclave ";
    // An array
    let word:[u8;4] = [82, 117, 115, 116];
    // An vector
    let word_vec:Vec<u8> = vec![32, 115, 116, 114, 105, 110, 103, 33];

    // Construct a string from &'static string
    let mut hello_string = String::from(rust_raw_string);

    // Iterate on word array
    for c in word.iter() {
        hello_string.push(*c as char);
    }

    // Rust style convertion
    hello_string += String::from_utf8(word_vec).expect("Invalid UTF-8")
                                               .as_str();

    // Ocall to normal world for output
    println!("{}", &hello_string);

    rsgx_unit_tests!(
tests::test::test_consts,
tests::test::test_scale_unscale,
tests::test::test_conj,
tests::test::test_inv,
|| should_panic!(tests::test::test_divide_by_zero_natural()),
tests::test::test_inv_zero,
tests::test::test_l1_norm,
tests::test::test_pow,
tests::test::test_to_string,
tests::test::test_string_formatting,
tests::test::test_hash,
tests::test::test_hashset,
tests::test::test_is_nan,
tests::test::test_is_nan_special_cases,
tests::test::test_is_infinite,
tests::test::test_is_finite,
tests::test::test_is_normal,
tests::test::test_from_str,
tests::test::test_from_str_radix,
tests::test::test_from_str_fail,
tests::test::test_sum,
tests::test::test_prod,
tests::test::test_zero,
tests::test::test_one,
tests::test::float::test_norm,
tests::test::float::test_arg,
tests::test::float::test_polar_conv,
tests::test::float::test_exp,
tests::test::float::test_ln,
tests::test::float::test_powc,
tests::test::float::test_powf,
tests::test::float::test_log,
tests::test::float::test_some_expf_cases,
tests::test::float::test_sqrt,
tests::test::float::test_sin,
tests::test::float::test_cos,
tests::test::float::test_tan,
tests::test::float::test_asin,
tests::test::float::test_acos,
tests::test::float::test_atan,
tests::test::float::test_sinh,
tests::test::float::test_cosh,
tests::test::float::test_tanh,
tests::test::float::test_asinh,
tests::test::float::test_acosh,
tests::test::float::test_atanh,
tests::test::float::test_exp_ln,
tests::test::float::test_trig_to_hyperbolic,
tests::test::float::test_trig_identities,
tests::test::float::test_hyperbolic_identites,
tests::test::complex_arithmetic::test_add,
tests::test::complex_arithmetic::test_sub,
tests::test::complex_arithmetic::test_mul,
tests::test::complex_arithmetic::test_mul_add_float,
tests::test::complex_arithmetic::test_mul_add,
tests::test::complex_arithmetic::test_div,
tests::test::complex_arithmetic::test_rem,
tests::test::complex_arithmetic::test_neg,
tests::test::complex_arithmetic::test_add,
tests::test::complex_arithmetic::test_sub,
tests::test::complex_arithmetic::test_mul,
tests::test::complex_arithmetic::test_div,
tests::test::complex_arithmetic::test_mul_add_float,
tests::test::complex_arithmetic::test_mul_add,
tests::test::real_arithmetic::test_add,
tests::test::real_arithmetic::test_sub,
tests::test::real_arithmetic::test_mul,
tests::test::real_arithmetic::test_div,
tests::test::real_arithmetic::test_add,
tests::test::real_arithmetic::test_sub,
tests::test::real_arithmetic::test_mul,
tests::test::real_arithmetic::test_rem,
tests::test::real_arithmetic::test_div,
tests::test::real_arithmetic::test_div_rem_gaussian,
cast::test_to_primitive,
cast::test_from_primitive,
cast::test_num_cast,
cast::test_as_primitive,
crand::standard_f64,
crand::generic_standard_f64,
crand::generic_uniform_f64,
crand::generic_mixed_f64,
crand::generic_uniform_i32,
);

    sgx_status_t::SGX_SUCCESS
}
