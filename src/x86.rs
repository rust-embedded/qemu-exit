// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2019-2020 Andre Richter <andre.o.richter@gmail.com>

//! x86_64 QEMU exit.

use crate::QemuExit;

/// QEMU exit code 1
pub const EXIT_FAILURE: u32 = 0; // as ((0 << 1) | 1) = 1

/// X86 QemuExit info struct
pub struct X86 {
    /// Port number of the isa-debug-exit device
    io_port: u16,
    /// As QEMU cannot exit(0), chose a value that represent succes code. It must be ood
    exit_success: u32,
}

impl X86 {
    /// Create a new X86 QemuExit struct
    pub fn new<T: Into<u16>, U: Into<u32>>(io_port: T, exit_success: U) -> Self {
        X86 {
            io_port: io_port.into(),
            exit_success: exit_success.into(),
        }
    }
}

impl QemuExit for X86 {
    /// QEMU binary executes `exit((code << 1) | 1)`.
    fn exit<T: Into<u32>>(&self, code: T) -> ! {
        use x86_64::instructions::port::Port;

        let mut port = Port::new(self.io_port);
        unsafe { port.write(code.into()) };

        // For the case that the QEMU exit attempt did not work, transition into an infinite loop.
        // Calling `panic!()` here is unfeasible, since there is a good chance this function here is
        // the last expression in the `panic!()` handler itself. This prevents a possible
        // infinite loop.
        loop {}
    }

    fn exit_success(&self) -> ! {
        let code = self.exit_success >> 1; // Shift because QEMU does ((code << 1) | 1)
        self.exit(code)
    }

    fn exit_failure(&self) -> ! {
        self.exit(EXIT_FAILURE)
    }
}
