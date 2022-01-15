#!/usr/bin/env bash

TIMEOUT="timeout 10"

if [[ $1 == *"aarch64"* ]]; then
    $TIMEOUT qemu-system-aarch64 -M raspi3 -display none -semihosting -kernel $1
elif [[ $1 == *"riscv64"* ]]; then
    $TIMEOUT qemu-system-riscv64 -M virt -bios tests/riscv64_virt/fw_jump.elf -display none -kernel $1
fi

let "status = $? - 13"
exit $status
