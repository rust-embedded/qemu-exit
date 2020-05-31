// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2019-2020 Andre Richter <andre.o.richter@gmail.com>

//! AArch64 QEMU exit.

/// QEMU exit code 0.
pub const EXIT_SUCCESS: u64 = 0;

/// QEMU exit code 1.
pub const EXIT_FAILURE: u64 = 1;

#[allow(non_upper_case_globals)]
const ADP_Stopped_ApplicationExit: u64 = 0x20026;

/// The parameter block layout that is expected by QEMU.
#[repr(C)]
struct qemu_parameter_block {
    arg0: u64,
    arg1: u64,
}

/// A Semihosting call using `0x18` - `SYS_EXIT`.
///
/// If QEMU finds `ADP_Stopped_ApplicationExit` in the first parameter, it uses the second parameter
/// as exit code.
///
/// If first paraemter != `ADP_Stopped_ApplicationExit`, exit code `1` is used.
fn semihosting_sys_exit_call(block: &qemu_parameter_block) -> ! {
    unsafe {
        llvm_asm!(
            "mov w0, 0x18
             mov x1, $0
             hlt #0xF000"
             : // No Outputs
             : "r"(block as *const _ as u64)
             : "w0", "x1", "memory"
             : "volatile"
        );
    }

    // For the case that the QEMU exit attempt did not work, transition into an infinite loop.
    // Calling `panic!()` here is unfeasible, since there is a good chance this function here is the
    // last expression in the `panic!()` handler itself. This prevents a possible infinite loop.
    loop {}
}

/// QEMU binary executes `exit(arg)`.
pub fn exit<T>(code: T) -> !
where
    T: Into<u64>,
{
    let block = qemu_parameter_block {
        arg0: ADP_Stopped_ApplicationExit,
        arg1: code.into(),
    };

    semihosting_sys_exit_call(&block)
}

/// QEMU binary executes `exit(0)`.
pub fn exit_success() -> ! {
    exit(EXIT_SUCCESS)
}

/// QEMU binary executes `exit(1)`.
pub fn exit_failure() -> ! {
    exit(EXIT_FAILURE)
}
