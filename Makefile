TARGET_AARCH64 := aarch64-unknown-none-softfloat
TARGET_RISCV64 := riscv64gc-unknown-none-elf

default:
	cargo build --target $(TARGET_AARCH64)
	cargo build --target $(TARGET_RISCV64)
	cargo build

clippy:
	cargo clippy --target $(TARGET_AARCH64)
	cargo build --target $(TARGET_RISCV64)
	cargo clippy

fmt:
	cargo fmt

ready: clippy fmt
	git pull
	cargo package --allow-dirty

clean:
	cargo clean
