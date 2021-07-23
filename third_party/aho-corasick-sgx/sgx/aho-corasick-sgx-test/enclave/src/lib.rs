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
use sgx_tunittest::*;

extern crate aho_corasick;

mod tests;

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

    rsgx_unit_tests!(tests::state_id_too_small,
                     tests::search_tests_have_unique_names,
tests::search_leftmost_longest::nfa_default,
tests::search_leftmost_longest::nfa_no_prefilter,
tests::search_leftmost_longest::nfa_all_sparse,
tests::search_leftmost_longest::nfa_all_dense,
tests::search_leftmost_longest::dfa_default,
tests::search_leftmost_longest::dfa_no_prefilter,
tests::search_leftmost_longest::dfa_all_sparse,
tests::search_leftmost_longest::dfa_all_dense,
tests::search_leftmost_longest::dfa_no_byte_class,
tests::search_leftmost_longest::dfa_no_premultiply,
tests::search_leftmost_longest::dfa_no_byte_class_no_premultiply,
tests::search_leftmost_first::nfa_default,
tests::search_leftmost_first::nfa_no_prefilter,
tests::search_leftmost_first::nfa_all_sparse,
tests::search_leftmost_first::nfa_all_dense,
tests::search_leftmost_first::dfa_default,
tests::search_leftmost_first::dfa_no_prefilter,
tests::search_leftmost_first::dfa_all_sparse,
tests::search_leftmost_first::dfa_all_dense,
tests::search_leftmost_first::dfa_no_byte_class,
tests::search_leftmost_first::dfa_no_premultiply,
tests::search_leftmost_first::dfa_no_byte_class_no_premultiply,
tests::search_standard_nonoverlapping::nfa_default,
tests::search_standard_nonoverlapping::nfa_no_prefilter,
tests::search_standard_nonoverlapping::nfa_all_sparse,
tests::search_standard_nonoverlapping::nfa_all_dense,
tests::search_standard_nonoverlapping::dfa_default,
tests::search_standard_nonoverlapping::dfa_no_prefilter,
tests::search_standard_nonoverlapping::dfa_all_sparse,
tests::search_standard_nonoverlapping::dfa_all_dense,
tests::search_standard_nonoverlapping::dfa_no_byte_class,
tests::search_standard_nonoverlapping::dfa_no_premultiply,
tests::search_standard_nonoverlapping::dfa_no_byte_class_no_premultiply,
tests::search_standard_overlapping_nfa_default,
tests::search_standard_overlapping_nfa_all_sparse,
tests::search_standard_overlapping_nfa_all_dense,
tests::search_standard_overlapping_dfa_default,
tests::search_standard_overlapping_dfa_all_sparse,
tests::search_standard_overlapping_dfa_all_dense,
tests::search_standard_overlapping_dfa_no_byte_class,
tests::search_standard_overlapping_dfa_no_premultiply,
tests::search_standard_overlapping_dfa_no_byte_class_no_premultiply,
tests::search_standard_stream_nfa_default,
tests::search_standard_stream_dfa_default,
tests::stream_not_allowed_leftmost_first,
tests::stream_not_allowed_leftmost_longest,
tests::overlapping_not_allowed_leftmost_first,
tests::overlapping_not_allowed_leftmost_longest
);

    sgx_status_t::SGX_SUCCESS
}
