#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate panic_halt;
extern crate nrf52840_hal;

use core;
use cortex_m_rt::{entry, exception};

static mut JUMP: Option<extern "C" fn()> = None;

pub fn boot_from(_scb: &mut cortex_m::peripheral::SCB, address: u32){
    unsafe {
        let stack_pointer = *(address as * const u32);
        let vector_table  = *((address + 4) as * const u32);

        cortex_m::asm::dsb();
        cortex_m::asm::isb();
        JUMP = Some(core::mem::transmute(vector_table));
        _scb.vtor.write(address);
        cortex_m::register::msp::write(stack_pointer);
        (JUMP.unwrap())();
    }
}

#[entry]
fn main() -> ! {
    let mut core_periphials = nrf52840_hal::target::CorePeripherals::take().unwrap() ;
    //let mut core = cortex_m::Periphials::take().unwrap();
    boot_from(&mut core_periphials.SCB, 0x4000);
    loop
    {
    }
}

#[exception]
fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
