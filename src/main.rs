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

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table is at: {:?}", level_4_page_table);

    let ptr = 0x2031b2 as *mut u8;

    // read from code page
    unsafe {
        let _x = *ptr;
    }
    println!("read worked");

    // write to a code page
    unsafe {
        *ptr = 42;
    }
    println!("write worked");

    // trying to triple fault
    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // stack_overflow();
    
    #[cfg(test)]
    test_main();

    println!("it id not crash!");
    tinyos::hlt_loop();
}

// existing panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    tinyos::hlt_loop();
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