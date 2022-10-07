#![no_std]
#![no_main]

//use macro provided in rt lib for type safety
use rt::entry;

entry!(main);

static RODATA: &[u8] = b"Hello, World!";
static mut BSS: u8 = 0;
static mut DATA: u16 = 1;

fn main() -> ! {
    let _x = RODATA;
    let _y = unsafe { &BSS };
    let _z = unsafe { &DATA };

    loop {}
}
