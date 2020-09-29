// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2019-2020 Andre Richter <andre.o.richter@gmail.com>

//! Exit QEMU with user-defined code.
//!
//! Quit a running QEMU session with user-defined exit code. Useful for unit or integration tests
//! using QEMU.
//!
//! ## TL;DR
//!
//! ```rust
//! use qemu_exit::QEMUExit;
//!
//! #[cfg(target_arch = "aarch64")]
//! let qemu_exit_handle = qemu_exit::AArch64::new();
//!
//! // addr: The address of sifive_test.
//! #[cfg(target_arch = "riscv64")]
//! let qemu_exit_handle = qemu_exit::RISCV64::new(addr);
//!
//! // io_base:             I/O-base of isa-debug-exit.
//! // custom_exit_success: A custom success code; Must be an odd number.
//! #[cfg(target_arch = "x86_64")]
//! let qemu_exit_handle = qemu_exit::X86::new(io_base, custom_exit_success);
//!
//! qemu_exit_handle.exit(1337);
//! qemu_exit_handle.exit_success();
//! qemu_exit_handle.exit_failure();
//! ```
//!
//! ## Architecture Specific Configuration
//!
//! ### AArch64
//!
//! Pass the `-semihosting` argument to QEMU invocation, e.g.:
//! ```
//! qemu-system-aarch64 -M raspi3 -serial stdio -semihosting -kernel kernel8.img
//! ```
//!
//! ### RISCV64
//!
//! You need to chose a machine with the `sifive_test` device, for exemple `-M virt`:
//! ```
//! qemu-system-riscv64 -M virt -nographic -monitor none -serial stdio -kernel kernel.elf
//! ```
//!
//! ### x86_64
//!
//! Add the special ISA debug exit device by passing the flags:
//! ```
//! -device isa-debug-exit,iobase=0xf4,iosize=0x04
//! ```
//!
//! When instantiating the handle, `iobase` must be given as the first parameter.
//!
//! The second parameter must be an `EXIT_SUCCESS` code of your choice that is an odd number, aka
//! bit number zero must be `1`. This is needed because in QEMU, the provided code is internally
//! binary-OR'ed with `0x1`. This is hardcoded and therefore, with `isa-debug-exit`, it is not
//! possible to let QEMU invoke `exit(0)`.
//!
//! ```rust
//! let qemu_exit_handle = qemu_exit::X86::new(io_base, custom_exit_success);
//! ```
//!
//! ## Literature
//!
//! - [Semihosting for AArch32 and AArch64](https://static.docs.arm.com/dui0003/b/semihosting.pdf)
//! - [QEMU isa-debug-exit source](https://git.qemu.org/?p=qemu.git;a=blob;f=hw/misc/debugexit.c)
//! - [QEMU sifive_test source](https://git.qemu.org/?p=qemu.git;a=blob;f=hw/misc/sifive_test.c)

#![deny(missing_docs)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(const_panic)]
#![no_std]

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(target_arch = "riscv64")]
pub mod riscv64;

#[cfg(target_arch = "riscv64")]
pub use riscv64::*;

#[cfg(target_arch = "x86_64")]
pub mod x86;

#[cfg(target_arch = "x86_64")]
pub use x86::*;

/// Generic interface for exiting QEMU.
pub trait QEMUExit {
    /// Exit with specified return code.
    ///
    /// Note: For `X86`, code is binary-OR'ed with `0x1` inside QEMU.
    fn exit(&self, code: u32) -> !;

    /// Exit QEMU using `EXIT_SUCCESS`, aka `0`, if possible.
    ///
    /// Note: Not possible for `X86`.
    fn exit_success(&self) -> !;

    /// Exit QEMU using `EXIT_FAILURE`, aka `1`.
    fn exit_failure(&self) -> !;
}
