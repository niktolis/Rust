#![no_std]
#![no_main]
// Used with nightly for trap generation (Testing exceptions)
#![feature(core_intrinsics)]

use core::intrinsics;

//use macro provided in rt lib for type safety
use rt::{entry, exception, ExceptionFrame};

entry!(main);

static RODATA: &[u8] = b"Hello, World!";
static mut BSS: u8 = 0;
static mut DATA: u16 = 1;

fn main() -> ! {
    let _x = RODATA;
    let _y = unsafe { &BSS };
    let _z = unsafe { &DATA };

    intrinsics::abort();
    //   loop {}
}

// #[no_mangle]
// pub extern "C" fn HardFault() -> ! {
//     loop {}
// }

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef)
}

exception!(*, default_handler);

fn default_handler(_irqn: i16) {
    loop {}
}
