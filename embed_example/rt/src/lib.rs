#![no_std]
#![deny(warnings)]

use core::fmt;
use core::panic::PanicInfo;
use core::ptr;

pub struct ExceptionFrame {
    // (General purpose) Register 0
    pub r0: u32,
}

impl fmt::Debug for ExceptionFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct Hex(u32);
        impl fmt::Debug for Hex {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "0x{:08x}", self.0)
            }
        }
        f.debug_struct("ExceptionFrame")
            .field("r0", &Hex(self.r0))
            .finish()
    }
}

// Reset Handler
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    // Initialize RAM
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static mut _sidata: u8;
    }

    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);

    // Call user entry point
    extern "Rust" {
        fn main() -> !;
    }

    main()
}

// The reset vector, a pointer into the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

// Enhance entry point (main) type safety for applications
#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            // type check the given path
            let f: fn() -> ! = $path;

            f()
        }
    };
}

pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

extern "C" {
    fn NMI();
    fn HardFault();
    fn MemManage();
    fn BusFault();
    fn UsageFault();
    fn SVCall();
    fn PendSV();
    fn SysTick();
}

/* Array of vectors (pointers to exception handlers) */
#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static __EXCEPTIONS: [Vector; 14] = [
    Vector { handler: NMI },
    Vector { handler: HardFault },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector {
        handler: UsageFault,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVCall },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: PendSV },
    Vector { handler: SysTick },
];

/* Default exception handler. Exceptions that have no assigned handler
by the user will make use of this. */
// #[no_mangle]
// pub extern "C" fn DefaultExceptionHandler() {
//     loop {}
// }

// Enhance exception type safety for applications
#[macro_export]
macro_rules! exception {
    (*, $handler:path) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)] // raise an error if this item is not accesible
        #[no_mangle]
        pub unsafe extern "C" fn DefaultHandler() {
            extern crate core;

            // validate the signature of the user provided handler
            let f: fn(i16) = $handler;

            const SCB_ICSR: *const u32 = 0xE000_ED04 as *const u32;

            // NOTE not volatile so the compiler can opt the load operation awa if the value is unused
            f(core::ptr::read(SCB_ICSR) as u8 as i16 - 16)
        }
    };

    (HardFault, $handler:path) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)] // raise an error if this item is not accessible
        #[no_mangle]
        pub unsafe extern "C" fn UserHardFault(ef: &$crate::ExceptionFrame) {
            // validate the signature of the user provided handler
            let f: fn(&$crate::ExceptionFrame) -> ! = $handler;

            f(ef)
        }
    };

    ($Name:ident, $handler:path,state: $State:ty = $initial_state:expr) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)] // raise an error if this item is not accessible
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            static mut STATE: $State = $initial_state;

            // check that this exception exists
            let _ = $crate::Exception::$Name;

            // validate the signature of the user provided handler
            let f: fn(&mut $State) = $handler;

            f(&mut STATE)
        }
    };

    ($Name:ident, $handler:path) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)] // raise an error if this item is not accessible
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            // check that this exception exists
            let _ = $crate::Exception::$Name;

            // validate the signature of the user provided handler
            let f: fn() = $handler;

            f()
        }
    };
}
