// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2019-2022 Andre Richter <andre.o.richter@gmail.com>

//! A simple test that exits QEMU with code 13.

#![no_main]
#![no_std]

use core::panic::PanicInfo;
use qemu_exit::QEMUExit;

//--------------------------------------------------------------------------------------------------
// AArch64
//--------------------------------------------------------------------------------------------------

#[cfg(target_arch = "aarch64")]
const QEMU_EXIT_HANDLE: qemu_exit::AArch64 = qemu_exit::AArch64::new();

#[cfg(target_arch = "aarch64")]
mod aarch64_raspi3;

//--------------------------------------------------------------------------------------------------
// RISCV64
//--------------------------------------------------------------------------------------------------

#[cfg(target_arch = "riscv64")]
const QEMU_EXIT_HANDLE: qemu_exit::RISCV64 = qemu_exit::RISCV64::new(0x100000);

#[cfg(target_arch = "riscv64")]
mod riscv64_virt;

//--------------------------------------------------------------------------------------------------
// x86
//--------------------------------------------------------------------------------------------------

#[cfg(target_arch = "x86_64")]
const QEMU_EXIT_HANDLE: qemu_exit::X86 = qemu_exit::X86::new(0xf4, 5);

//--------------------------------------------------------------------------------------------------
// Generic code
//--------------------------------------------------------------------------------------------------

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    QEMU_EXIT_HANDLE.exit_failure()
}

fn test_main() -> ! {
    QEMU_EXIT_HANDLE.exit(13)
}
