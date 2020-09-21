// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2019-2020 Andre Richter <estblcsk@gmail.com>
//

//! riscv64 QEMU exit.

use crate::QemuExit;

/// QEMU exit code 0.
pub const EXIT_SUCCESS: u32 = 0x5555;

/// QEMU failure flag.
pub const EXIT_FAILURE_FLAG: u32 = 0x3333;

/// QEMU exit code 1.
pub const EXIT_FAILURE: u32 = (0x1 << 16) | EXIT_FAILURE_FLAG;

/// QEMU reset.
pub const EXIT_RESET: u32 = 0x7777;

/// Riscv64 QemuExit info struct
pub struct Riscv64 {
    /// Address of the sifive_test mapped device
    addr: u64,
}

/// Encode the exit code using EXIT_FAILURE_FLAG
fn exit_code_encode(code: u32) -> u32 {
    (code << 16) | EXIT_FAILURE_FLAG
}

impl Riscv64 {
    /// Create a new Riscv64 QemuExit struct
    pub fn new<T: Into<u64>>(addr: u64) -> Self {
        Riscv64 { addr: addr.into() }
    }
}

impl QemuExit for Riscv64 {
    /// Exit qemu with specified exit code.
    fn exit<T: Into<u32>>(&self, code: T) -> ! {
        let mut code = code.into();

        // If code is not a special value, we need to encode it with EXIT_FAILURE_FLAG.
        if code != EXIT_SUCCESS && code != EXIT_FAILURE && code != EXIT_RESET {
            code = exit_code_encode(code);
        }

        unsafe {
            asm!(
                "sw {0}, 0({1})",
                in(reg)code, in(reg)self.addr
            );
        }

        loop {
            unsafe {
                // No need to bussy loop so wait for interrupt
                asm!("wfi");
            }
        }
    }

    /// QEMU exit with exit code 0
    fn exit_success(&self) -> ! {
        self.exit(EXIT_SUCCESS);
    }

    /// QEMU exit with exit code 1
    fn exit_failure(&self) -> ! {
        self.exit(EXIT_FAILURE);
    }
}
