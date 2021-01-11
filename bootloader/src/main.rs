#![no_std]
#![no_main]
#![feature(asm)]

use core::panic::PanicInfo;

mod riscv;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[link_section = ".init"]
#[no_mangle]
pub unsafe fn _init() -> ! {
    asm!(
        ".option push",
        ".option norelax",
        "    la gp, __global_pointer$",
        ".option pop"
    );

    // Reset any pending interrupts
    riscv::reset_pending_interrupts();

    // Disable all machine interrupts
    asm!("csrw mie, 0",);

    // Enable machine interrupts
    riscv::enable_machine_interrupts();

    {
        use riscv::MachineInterruptEnable as MIE;

        riscv::enable_interrupts(MIE::EXCEPTION_INTERRUPTS | MIE::SOFTWARE_INTERRUPTS);
    }

    // Clear the general purpose registers
    riscv::zero_gp_registers();

    main();
}

pub fn main() -> ! {
    loop {}
}
