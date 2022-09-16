//! RISC-V functionality

use core::arch::asm;

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

bitflags! {
    /// Machine status (`mstatus` CSR) bit flags.
    pub struct MachineStatus: u32 {
        /// User-mode interrupt enable
        const UIE = 1 << 0;
        /// Supervisor-mode interrupt enable
        const SIE = 1 << 1;
        /// Machine-mode interrupt enable
        const MIE = 1 << 3;

        /// Previous user-mode interrupt enable
        const UPIE = 1 << 4;
        /// Previous supervisor-mode interrupt enable
        const SPIE = 1 << 5;
        /// Previous machine-mode interrupt enable
        const MPIE = 1 << 7;

    }
}
/// Resets all general purpose registers to 0.
#[inline]
pub unsafe fn zero_gp_registers() {
    asm!(
        "li x1, 0",
        "li x2, 0",
        "li x3, 0",
        "li x4, 0",
        "li x5, 0",
        "li x6, 0",
        "li x7, 0",
        "li x8, 0",
        "li x9, 0",
        "li x10, 0",
        "li x11, 0",
        "li x12, 0",
        "li x13, 0",
        "li x14, 0",
        "li x15, 0",
        "li x16, 0",
        "li x17, 0",
        "li x18, 0",
        "li x19, 0",
        "li x20, 0",
        "li x21, 0",
        "li x22, 0",
        "li x23, 0",
        "li x24, 0",
        "li x25, 0",
        "li x26, 0",
        "li x27, 0",
        "li x28, 0",
        "li x29, 0",
        "li x30, 0",
        "li x31, 0",
    );
}

/// Enables machine interrupts.
#[inline]
pub fn enable_machine_interrupts() {
    unsafe { asm!("csrrs x0, mstatus, 1 << 3") }
}

/// Enables the given `interrupts` by setting them to 1 in the `mie` CSR.
#[inline]
pub fn enable_interrupts(interrupts: MachineInterruptEnable) {
    unsafe {
        asm!("csrrs x0, mie, {0}", in(reg) interrupts.bits);
    }
}

/// Resets pending machine interrupts.
#[inline]
pub fn reset_pending_interrupts() {
    unsafe {
        asm!("csrw mip, 0");
    }
}

/// Returns control to a debugging environment.
#[inline]
pub fn ebreak() {
    unsafe {
        asm!("ebreak");
    }
}
