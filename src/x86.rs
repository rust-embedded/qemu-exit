// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2019-2021 Andre Richter <andre.o.richter@gmail.com>

//! x86.

use crate::QEMUExit;

const EXIT_FAILURE: u32 = 0; // since ((0 << 1) | 1) = 1.

/// x86 configuration.
pub struct X86 {
    /// Port number of the isa-debug-exit device.
    io_base: u16,
    /// Since QEMU's isa-debug-exit cannot exit(0), choose a value that represents success for you.
    ///
    /// Note: Only odd values will work.
    custom_exit_success: u32,
}

/// Output a long on an io port
fn outl(io_base: u16, code: u32) {
    unsafe {
        asm!(
            "out dx, eax",
            in("dx") io_base,
            in("eax") code,
            options(nomem, nostack)
        );
    }
}

impl X86 {
    /// Create an instance.
    pub const fn new(io_base: u16, custom_exit_success: u32) -> Self {
        assert!((custom_exit_success & 1) == 1);

        X86 {
            io_base,
            custom_exit_success,
        }
    }
}

impl QEMUExit for X86 {
    fn exit(&self, code: u32) -> ! {
        outl(self.io_base, code); // QEMU will execute `exit(((code << 1) | 1))`.

        // For the case that the QEMU exit attempt did not work, transition into an infinite loop.
        // Calling `panic!()` here is unfeasible, since there is a good chance this function here is
        // the last expression in the `panic!()` handler itself. This prevents a possible infinite
        // loop.
        loop {
            unsafe {
                asm!("hlt", options(nomem, nostack));
            }
        }
    }

    fn exit_success(&self) -> ! {
        self.exit(self.custom_exit_success >> 1) // Shift because QEMU does ((code << 1) | 1).
    }

    fn exit_failure(&self) -> ! {
        self.exit(EXIT_FAILURE)
    }
}
