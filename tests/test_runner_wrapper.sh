#!/usr/bin/env bash

TIMEOUT="timeout 10"
if [[ $1 == *"aarch64"* ]]; then
    $TIMEOUT qemu-system-aarch64 -M raspi3b -display none -semihosting -kernel $1
elif [[ $1 == *"thumbv7em"* ]]; then
    arm-none-eabi-objcopy -O binary $1 /tmp/qemu_exit_armv7em.bin
    qemu-system-arm -m 16M -nographic -M mps2-an500 -cpu cortex-m7 -serial mon:stdio -semihosting -kernel /tmp/qemu_exit_armv7em.bin
elif [[ $1 == *"riscv64"* ]]; then
    $TIMEOUT qemu-system-riscv64 -M virt -bios tests/riscv64_virt/fw_jump.elf -display none -kernel $1
fi

let "status = $? - 13"
exit $status
