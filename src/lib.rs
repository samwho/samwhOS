#![feature(lang_items)]
#![feature(asm)]

#![no_std]

mod screen;
mod asm;

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[lang = "panic_fmt"]
extern fn rust_begin_panic() -> ! {
    loop {}
}

#[no_mangle]
pub extern fn kmain() -> ! {
    screen::init();
    screen::puts("Hello, world!\n");
    screen::puts("How goes?\n");

    loop { }
}

