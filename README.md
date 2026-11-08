# Badger OS

A modern, high-security, experimental operating system built for the **AArch64 (ARMv8-A)** architecture, featuring a **Rust-based microkernel** and **native Rust applications**.

## 🚀 Core Architecture

Badger OS is built on a strict "Defense-in-Depth" security model, combining software-level safety with hardware-enforced isolation:

* **Microkernel in Rust:** The privileged core (EL1) is kept minimal—handling only basic scheduling, Inter-Process Communication (IPC), and MMU management to maximize stability and fault tolerance.
* **Hardware Isolation (MMU):** The Memory Management Unit completely isolates user-space processes (EL0). Even if an application misbehaves or contains faulty `unsafe` code, it cannot compromise the kernel or other apps.
* **Monolanguage Ecosystem:** Both the kernel, system services/drivers, and user-space applications are written entirely in **Rust**, allowing for elegant, type-safe IPC and shared data structures without legacy C-ABI overhead.
* **Native Performance:** No interpreters or heavy runtimes—applications are compiled directly into native AArch64 machine code (`no_std`).

## 🛠️ Tech Stack

* **Language:** Rust (`no_std` with `aarch64-unknown-none`)
* **Target Architecture:** `AArch64` (ARMv8-A 64-bit)
* **Supported Platforms / Boards:** Raspberry Pi (3 / 4 / 5, Zero 2 W) & QEMU (`virt` / `raspi3b` / `raspi4b`)
* **Privilege Separation:** ARM Exception Levels (EL1 Kernel / EL0 User Space)

## 📂 Project Structure (Monorepo)

```text
├── kernel/         # The core microkernel (MMU, Scheduler, IPC, Board BSPs)
├── drivers/        # User-space device drivers
├── apps/           # Native Rust applications (`no_std`)
└── targets/        # Custom build targets, linker scripts & configurations
```

## 🚀 Getting Started

### 1. Prerequisites (Ubuntu / Debian)

To build and emulate Badger OS (`AArch64 / ARMv8-A`) on an Ubuntu host, install the QEMU system emulator and multi-architecture GDB debugger:

```bash
sudo apt update
sudo apt install qemu-system-arm gdb-multiarch
```

*(Note: The `qemu-system-arm` package includes the 64-bit `qemu-system-aarch64` binary).*

### 2. Rust Toolchain Setup

Add the bare-metal `AArch64` target and LLVM binary utilities to your Rust toolchain:

```bash
# Add the bare-metal 64-bit ARM target
rustup target add aarch64-unknown-none

# Install LLVM tools for inspecting ELF binaries and generating raw kernel images
rustup component add llvm-tools-preview
cargo install cargo-binutils
```

### 3. Emulating with QEMU

#### Generic `virt` Machine (Recommended for Kernel Development)
Emulates a generic ARMv8-A virtual board (`Cortex-A72`, GIC, PL011 UART, MMU):

```bash
qemu-system-aarch64 \
  -M virt \
  -cpu cortex-a72 \
  -smp 4 \
  -m 512M \
  -nographic \
  -kernel target/aarch64-unknown-none/debug/badger-kernel
```
*(Press `Ctrl+A` then `X` to exit QEMU).*

#### Raspberry Pi Emulation (`raspi3b` / `raspi4b`)
To test Raspberry Pi-specific peripherals (GPIO, Mini-UART / PL011, Mailbox):

```bash
qemu-system-aarch64 \
  -M raspi3b \
  -serial null -serial stdio \
  -display none \
  -kernel kernel8.img
```

### 4. Debugging with GDB

Append `-S -s` to your QEMU command to pause execution at startup and open a GDB server on port `1234`, then connect from a second terminal:

```bash
gdb-multiarch target/aarch64-unknown-none/debug/badger-kernel -ex "target remote :1234"
```
