// Copyright (c) 2018 FaultyRAM
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at
// your option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(not(target_env = "msvc"))]
use pkg_config;
use std::fmt::{self, Display, Formatter};
use std::{error, io};
#[cfg(all(target_os = "windows", target_env = "msvc"))]
use vcpkg;

#[derive(Debug)]
pub(crate) enum Error {
    BindgenFailed,
    IO(io::Error),
    #[cfg(not(target_env = "msvc"))]
    PkgConfig(pkg_config::Error),
    #[cfg(all(target_os = "windows", target_env = "msvc"))]
    Vcpkg(vcpkg::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::BindgenFailed => {
                f.write_str("could not generate bindings from the given headers")
            }
            Error::IO(e) => e.fmt(f),
            #[cfg(not(target_env = "msvc"))]
            Error::PkgConfig(e) => e.fmt(f),
            #[cfg(all(target_os = "windows", target_env = "msvc"))]
            Error::Vcpkg(e) => e.fmt(f),
        }
    }
}

impl error::Error for Error {}
