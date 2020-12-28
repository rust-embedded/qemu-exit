// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2019-2020 Andre Richter <andre.o.richter@gmail.com>

//! AArch64 specific setup code.

#[no_mangle]
unsafe fn _start() -> ! {
    asm!("mov sp, #0x80000");

    super::test_main()
}
