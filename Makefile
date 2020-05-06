TARGET_AARCH64 := aarch64-unknown-none-softfloat

default:
	cargo build --target $(TARGET_AARCH64)
	cargo build

clippy:
	cargo clippy --target $(TARGET_AARCH64)
	cargo clippy

fmt:
	cargo fmt

ready: clippy fmt
	git pull
	cargo package --allow-dirty

clean:
	cargo clean
