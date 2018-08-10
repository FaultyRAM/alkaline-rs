// Copyright (c) 2018 FaultyRAM
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at
// your option. This file may not be copied, modified, or distributed
// except according to those terms.

use error::Error;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LibraryInfo {
    include_dirs: Vec<PathBuf>,
}

impl LibraryInfo {
    pub(crate) fn find() -> Result<Self, Error> {
        #[cfg(not(target_os = "windows"))]
        {
            Self::find_with_pkg_config()
        }
        #[cfg(all(target_os = "windows", not(target_env = "msvc")))]
        {
            Self::find_with_pkg_config().or_else(|_| Self::find_with_winreg())
        }
        #[cfg(all(target_os = "windows", target_env = "msvc"))]
        {
            Self::find_with_vcpkg().or_else(|_| Self::find_with_winreg())
        }
    }

    pub(crate) fn get_header_path<P: AsRef<Path>>(&self, header: P) -> Result<PathBuf, Error> {
        for p in &self.include_dirs {
            let mut header_path = p.join(&header);
            if header_path.exists() {
                return Ok(header_path);
            }
            header_path = p.join("AL").join(&header);
            if header_path.exists() {
                return Ok(header_path);
            }
        }
        Err(Error::IO(io::Error::new(
            io::ErrorKind::NotFound,
            format!("could not locate {}", header.as_ref().display()),
        )))
    }
}

impl LibraryInfo {
    #[cfg(not(target_env = "msvc"))]
    fn find_with_pkg_config() -> Result<Self, Error> {
        use pkg_config::Config;

        Config::new()
            .probe("openal")
            .map_err(Error::PkgConfig)
            .map(|lib| Self {
                include_dirs: lib.include_paths,
            })
    }

    #[cfg(all(target_os = "windows", target_env = "msvc"))]
    fn find_with_vcpkg() -> Result<Self, Error> {
        use vcpkg::Config;

        Config::new()
            .lib_name("OpenAL32")
            .copy_dlls(false)
            .find_package("openal-soft")
            .map_err(Error::Vcpkg)
            .map(|lib| Self {
                include_dirs: lib.include_paths,
            })
    }

    #[cfg(target_os = "windows")]
    fn find_with_winreg() -> Result<Self, Error> {
        use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ, KEY_WOW64_32KEY};
        use winreg::RegKey;

        #[cfg(target_arch = "x86")]
        const IMPORT_LIB_DIR: &str = r"libs\Win32";
        #[cfg(target_arch = "x86_64")]
        const IMPORT_LIB_DIR: &str = r"libs\Win64";

        let flags = KEY_READ | KEY_WOW64_32KEY;
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        hklm.open_subkey_with_flags(
            r"SOFTWARE\Creative Labs\OpenAL 1.1 Software Development Kit",
            flags,
        ).and_then(|sdk_key| {
            sdk_key
                .enum_keys()
                .filter_map(|res| {
                    res.and_then(|version| {
                        sdk_key
                            .open_subkey_with_flags(&version, flags)
                            .and_then(|inst| inst.get_value("InstallDir"))
                            .map(|path| (version, path))
                    }).ok()
                }).fold::<Option<(String, String)>, _>(None, |a, b| {
                    if let Some(prev) = a {
                        if prev.0 >= b.0 {
                            return Some(prev);
                        }
                    }
                    Some(b)
                }).map(|(_, path)| {
                    println!(
                        "cargo:rustc-link-search=native={}",
                        Path::new(&path).join(IMPORT_LIB_DIR).display()
                    );
                    println!("cargo:rustc-link-lib=static=OpenAL32");
                    Self {
                        include_dirs: vec![Path::new(&path).join("include")],
                    }
                }).ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        "could not find OpenAL in the registry",
                    )
                })
        }).map_err(Error::IO)
    }
}
