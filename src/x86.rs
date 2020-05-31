// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2019-2020 Andre Richter <andre.o.richter@gmail.com>

//! x86_64 QEMU exit.

/// QEMU binary executes `exit((code << 1) | 1)`.
pub fn exit<T, const DEBUG_EXIT_IOBASE: u16>(code: T) -> !
where
    T: Into<u32>,
{
    use x86_64::instructions::port::Port;

    let mut port = Port::new(DEBUG_EXIT_IOBASE);
    unsafe { port.write(code.into()) };

    // For the case that the QEMU exit attempt did not work, transition into an infinite loop.
    // Calling `panic!()` here is unfeasible, since there is a good chance this function here is the
    // last expression in the `panic!()` handler itself. This prevents a possible infinite loop.
    loop {}
}
