#![no_std] //do not link the standard library
#![no_main]//disable the default entry point
use core::panic::PanicInfo;

//function called on panic, divergente functio never returns 
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


//printing hello word in the VGA text buffer
mod vga_buffer;

//overwriting the operating system entry point
#[no_mangle]//disabling name mangling to ensure that the compiler return a func named _start()
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop{}
}

