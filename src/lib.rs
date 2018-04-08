#![feature(lang_items)]
#![feature(asm)]

#![no_std]

mod screen;
mod asm;
mod grub;
mod math;

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

    let s = "11111111111111111111111111111111111111111111111111111111111111111111111111111111";
    screen::println(s);
    screen::new_line();
    screen::println("Hello!");
    screen::println("Hello!");
    screen::println("Hello!");
    screen::println("Hello!");
    screen::println("Hello!");


    // let mbh = grub::multiboot_header_from_addr(0x100000);
    // if !mbh.is_valid() {
    //     screen::println("grub multiboot header is invalid!");
    //     screen::puti(mbh.magic);
    //     screen::putc('\n');
    // }

    loop {}
}
