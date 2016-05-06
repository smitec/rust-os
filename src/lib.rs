#![feature(lang_items)]
#![no_std]
extern crate rlibc;

#[no_mangle]
pub extern fn rust_main() {
    let hello = b"Hello World!";
    let color = 0x1f;

    let mut hello_colored = [color; 24];
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i*2] = *char_byte;
    }
    
    let buffer_pointer = (0xb8000 + 1988) as *mut _;
    unsafe { *buffer_pointer = hello_colored };

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}
