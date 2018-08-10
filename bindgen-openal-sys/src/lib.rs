// Copyright (c) 2018 FaultyRAM
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at
// your option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Raw, machine-generated bindings to OpenAL.

#![no_std]
#![forbid(
    warnings,
    future_incompatible,
    rust_2018_idioms,
    unused,
    missing_copy_implementations,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_qualifications,
    unused_results
)]
#![deny(unused_qualifications)]
#![allow(bad_style)]
#![cfg_attr(
    feature = "cargo-clippy",
    forbid(
        clippy,
        clippy_pedantic,
        clippy_complexity,
        clippy_correctness,
        clippy_perf,
        clippy_style,
    )
)]

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
