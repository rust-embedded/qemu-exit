#!/usr/bin/env bash

rust-objcopy --strip-all -O binary $1 $1.img
TEST_BINARY=$(echo $1.img | sed -e 's/.*target/target/g')

if [[ $TEST_BINARY == *"aarch64"* ]]; then
    qemu-system-aarch64 -M raspi3 -display none -semihosting -kernel $TEST_BINARY
fi

let "status = $? - 13"
exit $status
