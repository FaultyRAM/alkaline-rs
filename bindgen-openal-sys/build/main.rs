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

use bindgen::callbacks::{IntKind, ParseCallbacks};
use error::Error;
use library_info::LibraryInfo;
use std::env;
use std::path::Path;

#[derive(Clone, Copy, Debug)]
struct CallbacksParser;

impl ParseCallbacks for CallbacksParser {
    fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
        match name {
            "AL_FALSE" | "AL_TRUE" => Some(IntKind::Custom {
                name: "ALboolean",
                is_signed: false,
            }),
            "ALC_FALSE" | "ALC_TRUE" => Some(IntKind::Custom {
                name: "ALCboolean",
                is_signed: false,
            }),
            _ if name.starts_with("AL_") => Some(IntKind::Custom {
                name: "ALenum",
                is_signed: true,
            }),
            _ if name.starts_with("ALC_") => Some(IntKind::Custom {
                name: "ALCenum",
                is_signed: true,
            }),
            _ => None,
        }
    }
}

fn main() -> Result<(), Error> {
    LibraryInfo::find()
        .and_then(|info| {
            let al_h = info.get_header_path("al.h")?;
            let alc_h = info.get_header_path("alc.h")?;
            bindgen::builder()
                .header(al_h.to_string_lossy())
                .header(alc_h.to_string_lossy())
                .generate_comments(false)
                .whitelist_recursively(false)
                .opaque_type("ALCdevice")
                .opaque_type("ALCcontext")
                .whitelist_type("ALCdevice")
                .whitelist_type("ALCcontext")
                .whitelist_type("ALboolean")
                .whitelist_type("ALchar")
                .whitelist_type("ALbyte")
                .whitelist_type("ALubyte")
                .whitelist_type("ALshort")
                .whitelist_type("ALushort")
                .whitelist_type("ALint")
                .whitelist_type("ALuint")
                .whitelist_type("ALsizei")
                .whitelist_type("ALenum")
                .whitelist_type("ALfloat")
                .whitelist_type("ALdouble")
                .whitelist_type("ALvoid")
                .whitelist_type("ALCboolean")
                .whitelist_type("ALCchar")
                .whitelist_type("ALCbyte")
                .whitelist_type("ALCubyte")
                .whitelist_type("ALCshort")
                .whitelist_type("ALCushort")
                .whitelist_type("ALCint")
                .whitelist_type("ALCuint")
                .whitelist_type("ALCsizei")
                .whitelist_type("ALCenum")
                .whitelist_type("ALCfloat")
                .whitelist_type("ALCdouble")
                .whitelist_type("ALCvoid")
                .whitelist_function("alDopplerFactor")
                .whitelist_function("alDopplerVelocity")
                .whitelist_function("alSpeedOfSound")
                .whitelist_function("alDistanceModel")
                .whitelist_function("alEnable")
                .whitelist_function("alDisable")
                .whitelist_function("alIsEnabled")
                .whitelist_function("alGetString")
                .whitelist_function("alGetBooleanv")
                .whitelist_function("alGetIntegerv")
                .whitelist_function("alGetFloatv")
                .whitelist_function("alGetDoublev")
                .whitelist_function("alGetBoolean")
                .whitelist_function("alGetInteger")
                .whitelist_function("alGetFloat")
                .whitelist_function("alGetDouble")
                .whitelist_function("alGetError")
                .whitelist_function("alIsExtensionPresent")
                .whitelist_function("alGetProcAddress")
                .whitelist_function("alGetEnumValue")
                .whitelist_function("alListenerf")
                .whitelist_function("alListener3f")
                .whitelist_function("alListenerfv")
                .whitelist_function("alListeneri")
                .whitelist_function("alListener3i")
                .whitelist_function("alListeneriv")
                .whitelist_function("alGetListenerf")
                .whitelist_function("alGetListener3f")
                .whitelist_function("alGetListenerfv")
                .whitelist_function("alGetListeneri")
                .whitelist_function("alGetListener3i")
                .whitelist_function("alGetListeneriv")
                .whitelist_function("alGenSources")
                .whitelist_function("alDeleteSources")
                .whitelist_function("alIsSource")
                .whitelist_function("alSourcef")
                .whitelist_function("alSource3f")
                .whitelist_function("alSourcefv")
                .whitelist_function("alSourcei")
                .whitelist_function("alSource3i")
                .whitelist_function("alSourceiv")
                .whitelist_function("alGetSourcef")
                .whitelist_function("alGetSource3f")
                .whitelist_function("alGetSourcefv")
                .whitelist_function("alGetSourcei")
                .whitelist_function("alGetSource3i")
                .whitelist_function("alGetSourceiv")
                .whitelist_function("alSourcePlayv")
                .whitelist_function("alSourceStopv")
                .whitelist_function("alSourceRewindv")
                .whitelist_function("alSourcePausev")
                .whitelist_function("alSourcePlay")
                .whitelist_function("alSourceStop")
                .whitelist_function("alSourceRewind")
                .whitelist_function("alSourcePause")
                .whitelist_function("alSourceQueueBuffers")
                .whitelist_function("alSourceUnqueueBuffers")
                .whitelist_function("alGenBuffers")
                .whitelist_function("alDeleteBuffers")
                .whitelist_function("alIsBuffer")
                .whitelist_function("alBufferData")
                .whitelist_function("alBufferf")
                .whitelist_function("alBuffer3f")
                .whitelist_function("alBufferfv")
                .whitelist_function("alBufferi")
                .whitelist_function("alBuffer3i")
                .whitelist_function("alBufferiv")
                .whitelist_function("alGetBufferf")
                .whitelist_function("alGetBuffer3f")
                .whitelist_function("alGetBufferfv")
                .whitelist_function("alGetBufferi")
                .whitelist_function("alGetBuffer3i")
                .whitelist_function("alGetBufferiv")
                .whitelist_function("alcCreateContext")
                .whitelist_function("alcMakeContextCurrent")
                .whitelist_function("alcProcessContext")
                .whitelist_function("alcSuspendContext")
                .whitelist_function("alcDestroyContext")
                .whitelist_function("alcGetCurrentContext")
                .whitelist_function("alcGetContextsDevice")
                .whitelist_function("alcOpenDevice")
                .whitelist_function("alcCloseDevice")
                .whitelist_function("alcGetError")
                .whitelist_function("alcIsExtensionPresent")
                .whitelist_function("alcGetProcAddress")
                .whitelist_function("alcGetEnumValue")
                .whitelist_function("alcGetString")
                .whitelist_function("alcGetIntegerv")
                .whitelist_function("alcCaptureOpenDevice")
                .whitelist_function("alcCaptureCloseDevice")
                .whitelist_function("alcCaptureStart")
                .whitelist_function("alcCaptureStop")
                .whitelist_function("alcCaptureSamples")
                .whitelist_var("AL_NONE")
                .whitelist_var("AL_FALSE")
                .whitelist_var("AL_TRUE")
                .whitelist_var("AL_SOURCE_RELATIVE")
                .whitelist_var("AL_CONE_INNER_ANGLE")
                .whitelist_var("AL_CONE_OUTER_ANGLE")
                .whitelist_var("AL_PITCH")
                .whitelist_var("AL_POSITION")
                .whitelist_var("AL_DIRECTION")
                .whitelist_var("AL_VELOCITY")
                .whitelist_var("AL_LOOPING")
                .whitelist_var("AL_BUFFER")
                .whitelist_var("AL_GAIN")
                .whitelist_var("AL_MIN_GAIN")
                .whitelist_var("AL_MAX_GAIN")
                .whitelist_var("AL_ORIENTATION")
                .whitelist_var("AL_SOURCE_STATE")
                .whitelist_var("AL_INITIAL")
                .whitelist_var("AL_PLAYING")
                .whitelist_var("AL_PAUSED")
                .whitelist_var("AL_STOPPED")
                .whitelist_var("AL_BUFFERS_QUEUED")
                .whitelist_var("AL_BUFFERS_PROCESSED")
                .whitelist_var("AL_REFERENCE_DISTANCE")
                .whitelist_var("AL_ROLLOFF_FACTOR")
                .whitelist_var("AL_CONE_OUTER_GAIN")
                .whitelist_var("AL_MAX_DISTANCE")
                .whitelist_var("AL_SEC_OFFSET")
                .whitelist_var("AL_SAMPLE_OFFSET")
                .whitelist_var("AL_BYTE_OFFSET")
                .whitelist_var("AL_SOURCE_OFFSET")
                .whitelist_var("AL_SOURCE_TYPE")
                .whitelist_var("AL_STATIC")
                .whitelist_var("AL_STREAMING")
                .whitelist_var("AL_UNDETERMINED")
                .whitelist_var("AL_FORMAT_MONO8")
                .whitelist_var("AL_FORMAT_MONO16")
                .whitelist_var("AL_FORMAT_STEREO8")
                .whitelist_var("AL_FORMAT_STEREO16")
                .whitelist_var("AL_FREQUENCY")
                .whitelist_var("AL_BITS")
                .whitelist_var("AL_CHANNELS")
                .whitelist_var("AL_SIZE")
                .whitelist_var("AL_NO_ERROR")
                .whitelist_var("AL_INVALID_NAME")
                .whitelist_var("AL_INVALID_ENUM")
                .whitelist_var("AL_INVALID_VALUE")
                .whitelist_var("AL_INVALID_OPERATION")
                .whitelist_var("AL_OUT_OF_MEMORY")
                .whitelist_var("AL_VENDOR")
                .whitelist_var("AL_VERSION")
                .whitelist_var("AL_RENDERER")
                .whitelist_var("AL_EXTENSIONS")
                .whitelist_var("AL_DOPPLER_FACTOR")
                .whitelist_var("AL_DOPPLER_VELOCITY")
                .whitelist_var("AL_SPEED_OF_SOUND")
                .whitelist_var("AL_DISTANCE_MODEL")
                .whitelist_var("AL_INVERSE_DISTANCE")
                .whitelist_var("AL_INVERSE_DISTANCE_CLAMPED")
                .whitelist_var("AL_LINEAR_DISTANCE")
                .whitelist_var("AL_LINEAR_DISTANCE_CLAMPED")
                .whitelist_var("AL_EXPONENT_DISTANCE")
                .whitelist_var("AL_EXPONENT_DISTANCE_CLAMPED")
                .whitelist_var("ALC_FALSE")
                .whitelist_var("ALC_TRUE")
                .whitelist_var("ALC_FREQUENCY")
                .whitelist_var("ALC_REFRESH")
                .whitelist_var("ALC_SYNC")
                .whitelist_var("ALC_MONO_SOURCES")
                .whitelist_var("ALC_STEREO_SOURCES")
                .whitelist_var("ALC_NO_ERROR")
                .whitelist_var("ALC_INVALID_DEVICE")
                .whitelist_var("ALC_INVALID_CONTEXT")
                .whitelist_var("ALC_INVALID_ENUM")
                .whitelist_var("ALC_INVALID_VALUE")
                .whitelist_var("ALC_OUT_OF_MEMORY")
                .whitelist_var("ALC_MAJOR_VERSION")
                .whitelist_var("ALC_MINOR_VERSION")
                .whitelist_var("ALC_ATTRIBUTES_SIZE")
                .whitelist_var("ALC_ALL_ATTRIBUTES")
                .whitelist_var("ALC_DEFAULT_DEVICE_SPECIFIER")
                .whitelist_var("ALC_DEVICE_SPECIFIER")
                .whitelist_var("ALC_EXTENSIONS")
                .whitelist_var("ALC_CAPTURE_DEVICE_SPECIFIER")
                .whitelist_var("ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER")
                .whitelist_var("ALC_CAPTURE_SAMPLES")
                .whitelist_var("ALC_DEFAULT_ALL_DEVICES_SPECIFIER")
                .whitelist_var("ALC_ALL_DEVICES_SPECIFIER")
                .use_core()
                .ctypes_prefix("libc")
                .parse_callbacks(Box::new(CallbacksParser))
                .rustfmt_bindings(true)
                .generate()
                .map_err(|_| Error::BindgenFailed)
        }).and_then(|bindings| {
            let out_dir = env::var_os("OUT_DIR").expect("`OUT_DIR` is not set");
            let path = Path::new(&out_dir).join("bindings.rs");
            bindings.write_to_file(path).map_err(Error::IO)
        })
}
