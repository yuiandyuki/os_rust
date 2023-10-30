#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
mod vga_buffer;
use core::panic::PanicInfo;

// This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) ->!{
    print!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() ->! {
    println!("Hello World{}", "!");
    panic!("Some panic message");
    loop {}
}

#[cfg(test)]
fn test_runner(_test: &[&dyn Fn()]) {
    loop {}
}
