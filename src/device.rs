// Copyright (c) 2018 FaultyRAM
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at
// your option. This file may not be copied, modified, or distributed
// except according to those terms.

use bindgen_openal_sys::ALCdevice;

#[derive(Debug)]
/// An audio device that supports either capture (recording) or playback.
pub struct Device(*mut ALCdevice);
