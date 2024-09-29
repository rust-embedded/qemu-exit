TARGET_AARCH64 := aarch64-unknown-none-softfloat
TARGET_AARCH32 := thumbv7em-none-eabi
LINKER_SCRIPT_AARCH64 := tests/aarch64_raspi3/link.ld
LINKER_SCRIPT_AARCH32 := tests/armv7m_mps2an500/link.ld

TARGET_RISCV64 := riscv64gc-unknown-none-elf
LINKER_SCRIPT_RISCV64 := tests/riscv64_virt/link.ld

default:
	cargo build --target $(TARGET_AARCH64) --release
	cargo build --target $(TARGET_AARCH32) --release
	cargo build --target $(TARGET_RISCV64) --release
	cargo build --release

clippy:
	cargo clippy --target $(TARGET_AARCH64)
	cargo clippy --target $(TARGET_AARCH32)
	cargo clippy --target $(TARGET_RISCV64)
	cargo clippy

test:
	RUSTFLAGS="-C link-arg=-T$(LINKER_SCRIPT_AARCH64)" \
                cargo test \
                --target $(TARGET_AARCH64) \
                --release
	RUSTFLAGS="-C link-arg=-T$(LINKER_SCRIPT_AARCH32) -Clink-args=-Map=/tmp/qemuexit-mapfile.map" \
                cargo test \
                --target $(TARGET_AARCH32) \
                --release
	RUSTFLAGS="-C link-arg=-T$(LINKER_SCRIPT_RISCV64)" \
                cargo test \
                --target $(TARGET_RISCV64) \
                --release

fmt:
	cargo fmt

ready: clippy fmt
	git pull
	cargo package --allow-dirty

clean:
	cargo clean
