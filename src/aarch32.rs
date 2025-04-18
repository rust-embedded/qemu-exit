// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2024 Philipp Schulz <schulz.phil@gmx.de>

//! AArch32.

use crate::QEMUExit;
use core::arch::asm;

const EXIT_SUCCESS: u32 = 0;
const EXIT_FAILURE: u32 = 1;

#[allow(non_upper_case_globals)]
const ADP_Stopped_ApplicationExit: u32 = 0x20026;

/// The parameter block layout that is expected by QEMU.
///
/// If QEMU finds `ADP_Stopped_ApplicationExit` in the first parameter, it uses the second parameter
/// as exit code.
///
/// If first paraemter != `ADP_Stopped_ApplicationExit`, exit code `1` is used.
#[repr(C)]
struct QEMUParameterBlock {
    arg0: u32,
    arg1: u32,
}

/// AArch32 configuration.
pub struct AArch32 {}

/// A Semihosting call using `0x20` - `SYS_EXIT_EXTENDED`.
fn semihosting_sys_exit_call(block: &QEMUParameterBlock) -> ! {
    unsafe {
        asm!(
            "bkpt #0xab",
            in("r0") 0x20,
            in("r1") block as *const _ as u32,
            options(nostack)
        );

        // For the case that the QEMU exit attempt did not work, transition into an infinite loop.
        // Calling `panic!()` here is unfeasible, since there is a good chance this function here is
        // the last expression in the `panic!()` handler itself. This prevents a possible
        // infinite loop.
        loop {
            asm!("wfe", options(nomem, nostack));
        }
    }
}

impl AArch32 {
    /// Create an instance.
    pub const fn new() -> Self {
        AArch32 {}
    }
}

impl QEMUExit for AArch32 {
    fn exit(&self, code: u32) -> ! {
        let block = QEMUParameterBlock {
            arg0: ADP_Stopped_ApplicationExit,
            arg1: code as u32,
        };

        semihosting_sys_exit_call(&block)
    }

    fn exit_success(&self) -> ! {
        self.exit(EXIT_SUCCESS)
    }

    fn exit_failure(&self) -> ! {
        self.exit(EXIT_FAILURE)
    }
}
