<p align="center">
  <img src="https://img.shields.io/badge/Bare%20Metal-OS-red?style=for-the-badge&logo=linux&logoColor=white" alt="Bare Metal Badge" />
</p>




<h1 align="center">
  Rusty x86_64 — Rust OS Kernel
</h1>

<p align="center">
  <img src="assets/rustyos.png" alt="Rust OS Boot Screen" width="45%" />
</p>




<p align="center">
  🚀 A tiny yet powerful <strong>bare-metal x86_64 Operating System</strong> written in <strong>Rust</strong>.  
  Featuring paging, heap allocation, CPU + hardware interrupts, VGA text rendering, serial output, and QEMU testing.
</p>

---

<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.73-orange?style=for-the-badge&logo=rust&logoColor=white" alt="Rust Badge" />
  <img src="https://img.shields.io/badge/Bare%20Metal-Enabled-black?style=for-the-badge&logo=terminal&logoColor=white" alt="Bare Metal Badge" />
  <img src="https://img.shields.io/badge/Cargo-Build%20System-orange?style=for-the-badge&logo=rust" alt="Cargo Badge" />
  <img src="https://img.shields.io/badge/no_std-Runtime-critical?style=for-the-badge&logo=rust&logoColor=white" alt="no_std Badge" />
  <img src="https://img.shields.io/badge/Architecture-x86__64-brightgreen?style=for-the-badge" alt="Architecture Badge" />
  <img src="https://img.shields.io/badge/Bootloader-Custom%20/Bootimage-blue?style=for-the-badge" alt="Bootloader Badge" />
  <img src="https://img.shields.io/badge/Memory-Paging%20%26%20Heap-blue?style=for-the-badge&logo=buffer" alt="Memory Badge" />
  <img src="https://img.shields.io/badge/GDT%20%26%20IDT-Configured-purple?style=for-the-badge" alt="GDT/IDT Badge" />
  <img src="https://img.shields.io/badge/Interrupts-CPU%20%2B%20Hardware-red?style=for-the-badge&logoColor=white" alt="Interrupts Badge" />
  <img src="https://img.shields.io/badge/Double%20Fault-Handled-important?style=for-the-badge" alt="Double Fault Badge" />
  <img src="https://img.shields.io/badge/Serial-COM1%20Output-yellow?style=for-the-badge" alt="Serial Badge" />
  <img src="https://img.shields.io/badge/VGA-Text%20Rendering-purple?style=for-the-badge" alt="VGA Badge" />
  <img src="https://img.shields.io/badge/QEMU-Tested-green?style=for-the-badge&logo=qemu" alt="QEMU Badge" />
  <img src="https://img.shields.io/badge/Unit%20Tests-In%20Kernel-lightgrey?style=for-the-badge" alt="Unit Tests Badge" />
  <img src="https://img.shields.io/badge/Shell-Scripts%20Support-informational?style=for-the-badge" alt="Shell Scripts Badge" />
  <img src="https://img.shields.io/badge/Allocators-Custom-orange?style=for-the-badge" alt="Allocators Badge" />
</p>



---

## About This Repo
<p align="center">
  <video width="70%" controls>
    <source src="assets/repo_vid.mp4" type="video/mp4">
    Your browser does not support the video tag.
  </video>
</p>

**Rusty x86_64** is a freestanding operating system kernel written in Rust.  
The project demonstrates how to build a kernel from scratch without relying on any standard library.  

Key learning objectives:

- Bare-metal kernel booting  
- CPU exceptions and double fault handling  
- Hardware interrupt handling (timer, keyboard)  
- Memory management: paging & custom heap allocators  
- VGA text mode and serial output  
- Kernel-level unit testing  
- Running inside QEMU

---
## 🧩 Features

- **Bare-metal support:** Runs directly on hardware / QEMU  
- **Custom boot screen:** ASCII Rust OS logo + progress indicators  
- **Memory management:** Paging and heap allocation with multiple allocator designs  
- **Interrupt handling:** CPU exceptions, double faults, hardware interrupts  
- **VGA + Serial I/O:** Safe interfaces for printing text  
- **Testing:** Kernel-level unit and integration tests  
- **Automation:** `run-qemu.sh` script for easy testing  
---
## 📂 Project Structure
```
├── Cargo.toml
├── .gitignore
├── x86_64-os.json
├── run-qemu.sh
├── src/
│ ├── allocator/
│ │ ├── bump.rs
│ │ ├── fixed_size_block.rs
│ │ └── linked_list.rs
│ ├── allocator.rs
│ ├── boot_screen.rs
│ ├── gdt.rs
│ ├── interrupts.rs
│ ├── lib.rs
│ ├── macros.rs
│ ├── main.rs
│ ├── memory.rs
│ ├── serial.rs
│ └── vga_buffer.rs
└── tests/
├── basic_boot.rs
├── interrupts.rs
├── serial_macros.rs
├── should_panic.rs
└── stack_overflow.rs
```
---
## 🛠 Build & Run
1. Install Rust and `bootimage`:
```bash
rustup component add rust-src llvm-tools-preview
cargo install bootimage
```
2. Build the kernel:
```bash
cargo bootimage
```
3. Run in QEMU
```bash
cargo run
```
---
## 🙏 Acknowledgements

- Huge thanks to **[Philipp Oppermann](https://os.phil-opp.com/)** for his excellent **“Writing an OS in Rust”** tutorial series, which inspired and guided this project.
