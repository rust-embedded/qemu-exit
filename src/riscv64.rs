// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2019-2020 Andre Richter <estblcsk@gmail.com>
//

//! riscv64 QEMU exit.

/// QEMU exit code 0.
pub const EXIT_SUCCESS: u64 = 0x5555;

/// QEMU failure flag.
pub const EXIT_FAILURE_FLAG: u64 = 0x3333;

/// QEMU exit code 1.
pub const EXIT_FAILURE: u64 = (0x1 << 16) | EXIT_FAILURE_FLAG;

/// QEMU reset.
pub const EXIT_RESET: u64 = 0x7777;

/// Encode the exit code using EXIT_FAILURE_FLAG
fn exit_code_encode(code: u64) -> u64 {
    (code << 16) | EXIT_FAILURE_FLAG
}

/// Exit qemu with specified exit code.
pub fn exit<T>(addr: T, code: T) -> !
where
    T: Into<u64>,
{
    let mut code = code.into();
    let addr = addr.into();

    // If code is not a special value, we need to encode it with EXIT_FAILURE_FLAG.
    if code != EXIT_SUCCESS && code != EXIT_FAILURE && code != EXIT_RESET {
        code = exit_code_encode(code);
    }

    unsafe {
        asm!(
            "sw {0}, 0({1})",
            in(reg)code, in(reg)addr
        );
    }

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

/// QEMU exit with exit code 0
pub fn exit_success<T>(addr: T) -> !
where
    T: Into<u64>,
{
    exit(addr.into(), EXIT_SUCCESS);
}

/// QEMU exit with exit code 1
pub fn exit_failure<T>(addr: T) -> !
where
    T: Into<u64>,
{
    exit(addr.into(), EXIT_FAILURE);
}
