#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(zz_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use zz_os::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use x86_64::registers::control::Cr3;
    println!("Hello World{}", "!");

    zz_os::init();

    let(level_4_poage_table, _) = Cr3::read();
    println!("Level 4 page table at {:?}", level_4_poage_table.start_address());

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    zz_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    zz_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    zz_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}