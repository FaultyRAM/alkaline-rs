// Copyright (c) 2018 FaultyRAM
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at
// your option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Build script for detecting and generating bindings to OpenAL.

#![forbid(
    warnings,
    future_incompatible,
    rust_2018_idioms,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_results
)]
#![deny(unused, unused_qualifications)]
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

extern crate bindgen;
#[cfg(not(target_env = "msvc"))]
extern crate pkg_config;
#[cfg(all(target_os = "windows", target_env = "msvc"))]
extern crate vcpkg;
#[cfg(target_os = "windows")]
extern crate winreg;

mod error;
mod library_info;

use error::Error;
use library_info::LibraryInfo;
use std::env;
use std::path::Path;

fn main() -> Result<(), Error> {
    LibraryInfo::find()
        .and_then(|info| {
            let al_h = info.get_header_path("al.h")?;
            let alc_h = info.get_header_path("alc.h")?;
            bindgen::builder()
                .use_core()
                .ctypes_prefix("libc")
                .header(al_h.to_string_lossy())
                .header(alc_h.to_string_lossy())
                .generate()
                .map_err(|_| Error::BindgenFailed)
        }).and_then(|bindings| {
            let out_dir = env::var_os("OUT_DIR").expect("`OUT_DIR` is not set");
            let path = Path::new(&out_dir).join("bindings.rs");
            bindings.write_to_file(path).map_err(Error::IO)
        })
}
