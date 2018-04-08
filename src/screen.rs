use asm;
use math;

const VGA_PTR: usize = 0xb8000;
const CHARS_PER_LINE: u8 = 80;
const NUM_LINES: u8 = 25;

static mut CURSOR: (u8, u8) = (0, 0);

pub fn init() {
    enable_cursor(0, NUM_LINES);
    clear();
}

pub fn clear() {
    for line in 0 .. NUM_LINES {
        blank_line(line);
    }
}

pub fn println(s: &str) {
    puts(s);
    putc('\n');
}

pub fn puts(s: &str) {
    for c in s.chars() {
        putc(c)
    }
}

pub fn puti(i: u32) { 
    let chars = "0123456789ABCDEF".as_bytes();
    puts("0x");
    putb(chars[((i >> 28) & 0xF) as usize]);
    putb(chars[((i >> 24) & 0xF) as usize]);
    putb(chars[((i >> 20) & 0xF) as usize]);
    putb(chars[((i >> 16) & 0xF) as usize]);
    putb(chars[((i >> 12) & 0xF) as usize]);
    putb(chars[((i >> 8)  & 0xF) as usize]);
    putb(chars[((i >> 4)  & 0xF) as usize]);
    putb(chars[( i        & 0xF) as usize]);
}

pub fn putc(c: char) {
    if c == '\n' {
        new_line();
    } else {
        unsafe {
            *screen_ptr(CURSOR.0, CURSOR.1) = c as u8;
        }
        advance_cursor();
    }

    update_cursor();
}

pub fn putb(b: u8) {
    putc(b as char);
}

fn advance_cursor() {
    unsafe {
        CURSOR = (CURSOR.0 + 1, CURSOR.1);
        if CURSOR.0 > CHARS_PER_LINE {
            new_line()
        }
    }
}

pub fn new_line() {
    unsafe {
        CURSOR = (0, math::min(CURSOR.1 + 1, NUM_LINES));
        if CURSOR.1 == NUM_LINES {
            scroll_up();
        }
    }
}

fn scroll_up() {
    for y in 1 .. NUM_LINES {
        for x in 0 .. CHARS_PER_LINE {
            unsafe {
                *screen_ptr(x, y - 1) = *screen_ptr(x, y);
            }
        }
    }

    blank_line(NUM_LINES);
}

fn blank_line(line: u8) {
    for x in 0 .. CHARS_PER_LINE {
       unsafe {
            *screen_ptr(x, line) = 0;
       }
    }
}

fn screen_ptr(x: u8, y: u8) -> *mut u8 {
    (VGA_PTR + (screen_offset_no_overflow(x, y) * 2)) as *mut u8
}

/// Calculates the offset into the screen buffer, casting arguments
/// to larger integers before doing so. In a previous incarnation
/// of this code we didn't do this, and ended up triggering an
/// integer overflow and writing to random parts of the screen.
///
/// "But Sam, doesn't Rust do nifty overflow checking for you?"
///
/// We are compiling in release mode, and release mode removes
/// those checks. We can't compile in debug mode, because that
/// appears to rely on libc.
fn screen_offset_no_overflow(x: u8, y: u8) -> usize {
    let wide_y: usize = y as usize;
    let wide_x: usize = x as usize;
    ((wide_y * (CHARS_PER_LINE as usize)) + wide_x) as usize
}

fn enable_cursor(start: u8, end: u8) {
    unsafe {
        asm::outb(0x3D4, 0x0A);
        asm::outb(0x3D5, (asm::inb(0x3D5) & 0xC0) | start);

        asm::outb(0x3D4, 0x0B);
        asm::outb(0x3D5, (asm::inb(0x3E0) & 0xE0) | end);
    }
}

fn update_cursor() {
    unsafe {
        let pos = screen_offset_no_overflow(CURSOR.0, CURSOR.1) as u16;

        asm::outb(0x3D4, 0x0F);
        asm::outb(0x3D5, (pos & 0xFF) as u8);
        asm::outb(0x3D4, 0x0E);
        asm::outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
    }
}
