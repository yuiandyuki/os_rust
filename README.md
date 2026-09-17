# os_rust

A small x86-64 kernel in Rust, built while working through Philipp Oppermann's [*Writing an OS in Rust*](https://os.phil-opp.com/). It boots in QEMU as a freestanding `no_std` binary.

## Implemented

- **VGA text output** (`vga_buffer.rs`) with `print!` / `println!` macros, and serial output (`serial.rs`) for test logs
- **CPU exceptions** (`interrupts.rs`, `gdt.rs`): IDT setup, breakpoint and page-fault handlers, and a double-fault handler running on a separate IST stack so that kernel stack overflows are caught
- **Hardware interrupts**: 8259 PIC remapping, timer and PS/2 keyboard handlers
- **Paging** (`memory.rs`): 4-level page table access through a physical-memory offset mapping, virtual → physical address translation, and a frame allocator built from the bootloader's memory map
- **Testing**: a custom `no_std` test framework that runs inside QEMU and reports over serial, with integration tests for boot, expected panics and stack overflow

## Run

```bash
rustup override set nightly
cargo install bootimage
cd os
cargo run    # boots the kernel in QEMU
cargo test   # runs the in-QEMU tests
```

Requires QEMU and a nightly Rust toolchain.
