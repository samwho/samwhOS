#![feature(lang_items)]
#![no_std]

mod screen;

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[lang = "panic_fmt"]
extern fn rust_begin_panic() -> ! {
    loop {}
}

#[no_mangle]
pub extern fn kmain() -> ! {
    screen::clear();
    screen::puts("Hello, world!\n");
    screen::puts("How goes?\n");

    loop { }
}

