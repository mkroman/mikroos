//! RISC-V functionality

use bitflags::bitflags;

bitflags! {
    /// Machine interrupt enable (`mie` CSR) bit flags.
    pub struct MachineInterruptEnable: u32 {
        /// User-mode software interrupt enable
        const USIE = 1 << 0;
        /// Supervisor-mode software interrupt enable
        const SSIE = 1 << 1;
        /// Machine-mode software interrupt enable
        const MSIE = 1 << 3;
        /// User-mode timer interrupt enable
        const UTIE = 1 << 4;
        /// Supervisor-mode timer interrupt enable
        const STIE = 1 << 5;
        /// Machine-mode timer interrupt enable
        const MTIE = 1 << 7;
        /// User-mode exception interrupt enable
        const UEIE = 1 << 8;
        /// Supervisor-mode exception interrupt enable
        const SEIE = 1 << 9;
        /// Machine-mode exception interrupt enable
        const MEIE = 1 << 11;

        /// User-, supervisor- and machine-mode exception interrupts
        const EXCEPTION_INTERRUPTS = Self::UEIE.bits | Self::SEIE.bits | Self::MEIE.bits;
        /// User-, supervisor- and machine-mode software interrupts
        const SOFTWARE_INTERRUPTS = Self::USIE.bits | Self::SSIE.bits | Self::MSIE.bits;
        /// User-, supervisor- and machine-mode timer interrupts
        const TIMER_INTERRUPTS = Self::UTIE.bits | Self::STIE.bits | Self::MTIE.bits;
    }
}

/// Resets all general purpose registers to 0.
#[inline(always)]
pub unsafe fn zero_gp_registers() {
    asm!(
        "lui x1, 0",
        "lui x2, 0",
        "lui x3, 0",
        "lui x4, 0",
        "lui x5, 0",
        "lui x6, 0",
        "lui x7, 0",
        "lui x8, 0",
        "lui x9, 0",
        "lui x10, 0",
        "lui x11, 0",
        "lui x12, 0",
        "lui x13, 0",
        "lui x14, 0",
        "lui x15, 0",
        "lui x16, 0",
        "lui x17, 0",
        "lui x18, 0",
        "lui x19, 0",
        "lui x20, 0",
        "lui x21, 0",
        "lui x22, 0",
        "lui x23, 0",
        "lui x24, 0",
        "lui x25, 0",
        "lui x26, 0",
        "lui x27, 0",
        "lui x28, 0",
        "lui x29, 0",
        "lui x30, 0",
        "lui x31, 0",
    );
}

/// Enables machine interrupts.
#[inline(always)]
pub fn enable_machine_interrupts() {
    unsafe { asm!("csrrs x0, mstatus, 1 << 3") }
}

/// Enables the given `interrupts` by setting them to 1 in the `mie` CSR.
#[inline(always)]
pub fn enable_interrupts(interrupts: MachineInterruptEnable) {
    unsafe {
        asm!("csrrs x0, mie, t0", in("t0") interrupts.bits());
    }
}

/// Resets pending machine interrupts.
#[inline(always)]
pub fn reset_pending_interrupts() {
    unsafe {
        asm!("csrw mip, 0");
    }
}
