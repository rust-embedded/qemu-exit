// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2019 Andre Richter <andre.o.richter@gmail.com>

//! x86 stubs for other architectures.
//!
//! Prevents errors in RLS-based IDEs running on x86 hosts, because RLS can't cross-compile yet.

#![allow(unused_variables)]
#![allow(missing_docs)]

pub mod aarch64 {
    pub fn exit<T>(code: T) -> !
    where
        T: Into<u64>,
    {
        unimplemented!()
    }

    pub fn exit_success() -> ! {
        unimplemented!()
    }

    pub fn exit_failure() -> ! {
        unimplemented!()
    }
}
