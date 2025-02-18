#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tinyos::test_runner)]
#![reexport_test_harness_main = "test_main"]



use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
mod vga_buffer;
mod serial;

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use tinyos::memory::active_level_4_table;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    tinyos::init();

   let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
   let l4_table = unsafe {
    active_level_4_table(phys_mem_offset)
   };

   for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
   }
    
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