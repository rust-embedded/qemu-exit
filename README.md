[![crates.io](https://img.shields.io/crates/d/qemu-exit.svg)](https://crates.io/crates/qemu-exit)
[![crates.io](https://img.shields.io/crates/v/qemu-exit.svg)](https://crates.io/crates/qemu-exit)
![Build](https://github.com/andre-richter/qemu-exit/workflows/Build/badge.svg)
![Test](https://github.com/andre-richter/qemu-exit/workflows/Test/badge.svg)

# qemu-exit

Exit QEMU with user-defined code.

Quit a running QEMU session with user-defined exit code. Useful for unit or integration tests using
QEMU.

## TL;DR

```rust
use qemu_exit::QEMUExit;

#[cfg(target_arch = "aarch64")]
let qemu_exit_handle = qemu_exit::AArch64::new();

// addr: The address of sifive_test.
#[cfg(target_arch = "riscv64")]
let qemu_exit_handle = qemu_exit::RISCV64::new(addr);

// io_base:             I/O-base of isa-debug-exit.
// custom_exit_success: A custom success code; Must be an odd number.
#[cfg(target_arch = "x86_64")]
let qemu_exit_handle = qemu_exit::X86::new(io_base, custom_exit_success);

qemu_exit_handle.exit(1337);
qemu_exit_handle.exit_success();
qemu_exit_handle.exit_failure();
```

## Architecture Specific Configuration

### AArch64

Pass the `-semihosting` argument to QEMU invocation, e.g.:
```
qemu-system-aarch64 -M raspi3 -serial stdio -semihosting -kernel kernel8.img
```

### RISCV64

You need to chose a machine with the `sifive_test` device, for exemple `-M virt`:
```
qemu-system-riscv64 -M virt -nographic -monitor none -serial stdio -kernel kernel.elf
```

### x86_64

Add the special ISA debug exit device by passing the flags:
```
-device isa-debug-exit,iobase=0xf4,iosize=0x04
```

When instantiating the handle, `iobase` must be given as the first parameter.

The second parameter must be an `EXIT_SUCCESS` code of your choice that is an odd number, aka
bit number zero must be `1`. This is needed because in QEMU, the provided code is internally
binary-OR'ed with `0x1`. This is hardcoded and therefore, with `isa-debug-exit`, it is not
possible to let QEMU invoke `exit(0)`.

```rust
let qemu_exit_handle = qemu_exit::X86::new(io_base, custom_exit_success);
```

## Literature

- [Semihosting for AArch32 and AArch64](https://static.docs.arm.com/dui0003/b/semihosting.pdf)
- [QEMU isa-debug-exit source](https://git.qemu.org/?p=qemu.git;a=blob;f=hw/misc/debugexit.c)
- [QEMU sifive_test source](https://git.qemu.org/?p=qemu.git;a=blob;f=hw/misc/sifive_test.c)

## Authors

- [**@andre-richter** Andre Richter](https://github.com/andre-richter)
- [**@Skallwar** Esteban Blanc](https://github.com/Skallwar)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

See the [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) for more information about utilized third
party projects and their respective licenses.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
