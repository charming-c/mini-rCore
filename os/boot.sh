#!/bin/bash
debug=""
if [ "$1" = "debug" ];then 
    debug="-s -S"
fi
echo $debug &&
cargo build --release &&
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin &&
qemu7-system-riscv64     -machine virt     -nographic     -bios ../bootloader/rustsbi-qemu.bin     -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 $debug