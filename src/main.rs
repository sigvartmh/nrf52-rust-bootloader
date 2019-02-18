#![no_std]
#![no_main]

extern crate cortex_m_rt;
extern crate cortex_m;
extern crate cortex_m_semihosting;
extern crate panic_halt;
extern crate nrf52840;

//extern crate nrf52840_hal;

use core;
use cortex_m_rt::{entry, exception};
use cortex_m::peripheral::{SCB, Peripherals};
//use nrf52840::nrf52840;
//use nrf52840_hal::nrf52840;

//pub type Result<T> = core::result::Result<T, Error>;

/*
!#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        use cortex_m;
        use cortex_m_semihosting;
        if unsafe { (*cortex_m::peripheral::DCB::ptr()).dhcsr.read() & 1 == 1 } {
            match cortex_m_semihosting::hio::hstdout() {
                Ok(mut stdout) => {write!(stdout, $($arg)*).ok();},
                Err(_) => ()
            };
        }
    })
}

!#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}
*/

static mut JUMP: Option<extern "C" fn()> = None;

pub fn boot_from(_scb: &mut cortex_m::peripheral::SCB, address: u32){
    unsafe {
        let arm_vector_table_offset = 4;
        let stack_pointer = *(address as * const u32);
        let vector_table  = *((address + arm_vector_table_offset) as * const u32);

        JUMP = Some(core::mem::transmute(vector_table));
        _scb.vtor.write(address);
        cortex_m::register::msp::write(stack_pointer);
        cortex_m::asm::dsb();
        cortex_m::asm::isb();
        (JUMP.unwrap());
    }
}

#[entry]
fn main() -> ! {
    //let mut core_periphials = nrf52840::CorePeriphials::take().unwrap();
    //let mut core = cortex_m::Periphials::take().unwrap();
    let mut core = Peripherals::take().unwrap();
    boot_from(&mut core.SCB, 0x4000);
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
