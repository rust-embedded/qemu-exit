[![crates.io](https://img.shields.io/crates/d/qemu-exit.svg)](https://crates.io/crates/qemu-exit)
[![crates.io](https://img.shields.io/crates/v/qemu-exit.svg)](https://crates.io/crates/qemu-exit)

# qemu-exit

Exit QEMU with user-defined code.

Quit a running QEMU session with user-defined exit code. Useful for unit or integration tests using
QEMU.

# AArch64

Pass the `-semihosting` argument to QEMU invocation, e.g.:
```
qemu-system-aarch64 -M raspi3 -serial stdio -semihosting -kernel kernel8.img
```

## Examples

Exit the QEMU session from anywhere in your code:
```rust
qemu_exit::aarch64::exit_success() // QEMU binary executes `exit(0)`.
qemu_exit::aarch64::exit_failure() // QEMU binary executes `exit(1)`.
qemu_exit::aarch64::exit(arg)      // Use a custom code. Argument must implement `Into<u64>`.
```
# RISCV64

You need to chose a machine with the sifive_test device, for exemple ``-M virt``

## Examples

Exit the QEMU session from anywhere in your code:
```rust
qemu_exit::riscv64::exit_success() // QEMU binary executes `exit(0)`.
qemu_exit::riscv64::exit_failure() // QEMU binary executes `exit(1)`.
qemu_exit::riscv64::exit(arg)      // Use a custom code. Argument must implement `Into<u64>`.
```

# x86_64

Add the special ISA debug exit device by passing the flags:
```
-device isa-debug-exit,iobase=0xf4,iosize=0x04
```

## Examples

The iobase is given as a `const generic`:
```rust
qemu_exit::x86::exit<{ 0xf4 }>(arg) // Use a custom code. Argument must implement `Into<u32>`.
```

### Note

The QEMU binary will execute `exit((arg << 1) | 1)`. This is hardcoded in the
QEMU sources. Therefore, with `isa-debug-exit`, it is not possible to let QEMU
invoke `exit(0)`.

## Literature

- [Semihosting for AArch32 and AArch64](https://static.docs.arm.com/dui0003/b/semihosting.pdf)
- [QEMU isa-debug-exit source](https://git.qemu.org/?p=qemu.git;a=blob;f=hw/misc/debugexit.c)
- [QEMU sifive_test source](https://git.qemu.org/?p=qemu.git;a=blob;f=hw/misc/sifive_test.c)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
