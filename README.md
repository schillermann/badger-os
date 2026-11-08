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

*(Instructions for building and booting via SD card on your target hardware will go here as development progresses.)*