// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2019-2022 Esteban Blanc <estblcsk@gmail.com>

//! RISCV64 specific setup code.

use core::arch::asm;

#[no_mangle]
unsafe fn _start() -> ! {
    asm!("la sp, _stack");

    super::test_main()
}
