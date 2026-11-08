#![no_std]
#![no_main]

use core::arch::global_asm;
use core::fmt::{self, Write};
use core::panic::PanicInfo;

// Minimal AArch64 boot stub:
// 1. Park secondary CPU cores (only core 0 proceeds).
// 2. Set the stack pointer to `__stack_top`.
// 3. Zero the `.bss` section.
// 4. Jump to `kernel_main`.
global_asm!(
    r#"
    .section .text._start
    .global _start
    .type _start, @function
_start:
    // Read MPIDR_EL1 and check core ID (Aff0)
    mrs     x0, mpidr_el1
    and     x0, x0, #0x3
    cbnz    x0, .Lpark_core

    // Set up stack pointer
    ldr     x0, =__stack_top
    mov     sp, x0

    // Clear .bss section
    ldr     x0, =__bss_start
    ldr     x1, =__bss_end
.Lbss_loop:
    cmp     x0, x1
    b.hs    .Lbss_done
    str     xzr, [x0], #8
    b       .Lbss_loop

.Lbss_done:
    bl      kernel_main

.Lpark_core:
    wfe
    b       .Lpark_core
"#
);

/// PL011 UART base address on the QEMU `virt` machine.
const UART0_BASE: usize = 0x0900_0000;
/// UART Data Register offset.
const UART_DR: usize = 0x00;
/// UART Flag Register offset.
const UART_FR: usize = 0x18;
/// Transmit FIFO full flag (TXFF, bit 5).
const UART_FR_TXFF: u32 = 1 << 5;

pub struct Pl011Uart {
    base: usize,
}

impl Pl011Uart {
    pub const fn new(base: usize) -> Self {
        Self { base }
    }

    pub fn putc(&self, byte: u8) {
        unsafe {
            let dr = (self.base + UART_DR) as *mut u32;
            let fr = (self.base + UART_FR) as *const u32;

            // Wait while the Transmit FIFO is full
            while core::ptr::read_volatile(fr) & UART_FR_TXFF != 0 {
                core::hint::spin_loop();
            }

            core::ptr::write_volatile(dr, byte as u32);
        }
    }

    pub fn puts(&self, s: &str) {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.putc(b'\r');
            }
            self.putc(byte);
        }
    }
}

impl Write for Pl011Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.puts(s);
        Ok(())
    }
}

fn current_el() -> u64 {
    let el: u64;
    unsafe {
        core::arch::asm!("mrs {}, CurrentEL", out(reg) el, options(nomem, nostack, preserves_flags));
    }
    (el >> 2) & 0x3
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let mut uart = Pl011Uart::new(UART0_BASE);

    let _ = writeln!(uart, "\n========================================");
    let _ = writeln!(uart, "  Badger OS - AArch64 Microkernel");
    let _ = writeln!(uart, "========================================");
    let _ = writeln!(uart, "[BOOT] _start initialized stack & .bss");
    let _ = writeln!(uart, "[UART] PL011 active at {:#010x}", UART0_BASE);
    let _ = writeln!(uart, "[CPU ] Running in Exception Level EL{}", current_el());
    let _ = writeln!(uart, "[OK  ] Kernel up and running (WFE loop).");

    loop {
        unsafe {
            core::arch::asm!("wfe", options(nomem, nostack, preserves_flags));
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut uart = Pl011Uart::new(UART0_BASE);
    let _ = writeln!(uart, "\n[PANIC] {}", info);
    loop {
        unsafe {
            core::arch::asm!("wfe", options(nomem, nostack, preserves_flags));
        }
    }
}
