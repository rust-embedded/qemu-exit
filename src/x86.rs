// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2019 Andre Richter <andre.o.richter@gmail.com>

//! x86_64 QEMU exit.

/// QEMU binary executes `exit((code << 1) | 1)`.
pub fn exit<T, const DEBUG_EXIT_IOBASE: u16>(code: T) -> !
where
    T: Into<u32>,
{
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(DEBUG_EXIT_IOBASE);
        port.write(code.into());

        core::intrinsics::unreachable()
    }
}
