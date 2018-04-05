const VGA_PTR: usize = 0xb8000;
const CHARS_PER_LINE: u32 = 80;
const NUM_LINES: u32 = 32;

static mut CURSOR: (u32, u32) = (0, 0);

pub fn clear() {
    for line in 0 .. NUM_LINES {
        blank_line(line);
    }
}

pub fn puts(s: &str) {
    for c in s.chars() {
        putc(c)
    }
}

fn putc(c: char) {
    if (c == '\n') {
        new_line();
        return;
    }

    unsafe {
        *screen_ptr(CURSOR) = c as u8;
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
                let screen_to = screen_ptr((x, y - 1));
                let screen_from = screen_ptr((x, y));
                *screen_to = *screen_from;
            }
        }
    }

    blank_line(NUM_LINES);
}

fn blank_line(line: u32) {
    for x in 0 .. CHARS_PER_LINE {
       unsafe {
            *screen_ptr((x, line)) = 0;
       }
    }
}

fn screen_ptr(pos: (u32, u32)) -> *mut u8 {
    let offset: usize = ((pos.1 * CHARS_PER_LINE) + pos.0) as usize;
    (VGA_PTR + (offset * 2)) as *mut u8
}
