#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tinyos::test_runner)]
#![reexport_test_harness_main = "test_main"]



use core::panic::PanicInfo;

mod vga_buffer;
mod serial;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    tinyos::init();

    // triggering a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // }

    // trying to triple fault
    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();
    
    #[cfg(test)]
    test_main();

    println!("it id not crash!");
    loop{}
}

// existing panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    tinyos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}