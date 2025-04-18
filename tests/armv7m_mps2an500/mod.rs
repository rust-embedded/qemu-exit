// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2024 Philipp Schulz <schulz.phil@gmx.de>

//! AArch32 specific setup code.

use core::arch::global_asm;

global_asm!(include_str!("./startup.S"));

#[no_mangle]
extern "C" fn entry() {
    super::test_main()
}
