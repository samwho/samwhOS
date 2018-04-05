use asm;

const VGA_PTR: usize = 0xb8000;
const CHARS_PER_LINE: u8 = 80;
const NUM_LINES: u8 = 15;

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

pub fn puts(s: &str) {
    for c in s.chars() {
        putc(c)
    }
    update_cursor();
}

fn putc(c: char) {
    if c == '\n' {
        new_line();
        return;
    }

    unsafe {
        *screen_ptr(CURSOR.0, CURSOR.1) = c as u8;
    }

    advance_cursor()
}

fn advance_cursor() {
    unsafe {
        if CURSOR.0 == CHARS_PER_LINE {
            new_line()
        } else {
            CURSOR = (CURSOR.0 + 1, CURSOR.1);
        }
    }
}

fn new_line() {
    unsafe {
        CURSOR = (0, CURSOR.1 + 1);
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
    let offset: usize = ((y * CHARS_PER_LINE) + x) as usize;
    (VGA_PTR + (offset * 2)) as *mut u8
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
        let pos = (CURSOR.1 * CHARS_PER_LINE + CURSOR.0) as u16;

        asm::outb(0x3D4, 0x0F);
        asm::outb(0x3D5, (pos & 0xFF) as u8);
        asm::outb(0x3D4, 0x0E);
        asm::outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
    }
}
