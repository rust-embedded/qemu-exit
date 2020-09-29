#!/usr/bin/env bash

TIMEOUT="timeout 10"

if [[ $1 == *"aarch64"* ]]; then
    rust-objcopy --strip-all -O binary $1 $1.img
    STRIPPED_BINARY=$(echo $1.img | sed -e 's/.*target/target/g')

    $TIMEOUT qemu-system-aarch64 -M raspi3 -display none -semihosting -kernel $STRIPPED_BINARY
elif [[ $1 == *"riscv64"* ]]; then
    $TIMEOUT qemu-system-riscv64 -M virt -bios tests/riscv64_virt/fw_jump.elf -display none -kernel $1
fi

let "status = $? - 13"
exit $status
