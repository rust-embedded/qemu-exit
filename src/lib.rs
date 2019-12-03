// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2019 Andre Richter <andre.o.richter@gmail.com>

//! Exit QEMU with user-defined code.
//!
//! Quit a running QEMU session with user-defined exit code. Useful for unit or integration tests
//! using QEMU.
//!
//! # AArch64
//!
//! Pass the `-semihosting` argument to QEMU invocation, e.g.:
//! ```
//! qemu-system-aarch64 -M raspi3 -serial stdio -semihosting -kernel kernel8.img
//! ```
//!
//! ## Examples
//!
//! Exit the QEMU session from anywhere in your code:
//! ```
//! qemu_exit::aarch64::exit_success() // QEMU binary executes `exit(0)`.
//! qemu_exit::aarch64::exit_failure() // QEMU binary executes `exit(1)`.
//! qemu_exit::aarch64::exit(arg)      // Use a custom code. Argument must implement `Into<u64>`.
//! ```
//!
//! # x86_64
//!
//! Add the special ISA debug exit device by passing the flags:
//! ```
//! -device isa-debug-exit,iobase=0xf4,iosize=0x04
//! ```
//!
//! ## Examples
//!
//! Iobase is configurable and used as a `const generic`:
//! ```
//! qemu_exit::x86::exit<{ 0xf4 }>(arg) // Use a custom code. Argument must implement `Into<u32>`.
//! ```
//! ### Note
//!
//! The QEMU binary will execute `exit((arg << 1) | 1)`. The is hardcoded in the QEMU sources.
//! Therefore, with `isa-debug-exit`, it is not possible to let QEMU invoke `exit(0)`.
//!
//! # Literature
//!
//!  - [Semihosting for AArch32 and AArch64](https://static.docs.arm.com/dui0003/b/semihosting.pdf)
//!  - [QEMU isa-debug-exit source](https://git.qemu.org/?p=qemu.git;a=blob;f=hw/misc/debugexit.c)

#![allow(incomplete_features)]
#![deny(missing_docs)]
#![feature(asm)]
#![feature(const_generics)]
#![feature(core_intrinsics)]
#![no_std]

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "x86_64")]
pub mod x86;

#[cfg(target_arch = "x86_64")]
mod host_stubs;

#[cfg(target_arch = "x86_64")]
pub use host_stubs::*;
