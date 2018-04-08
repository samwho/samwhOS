const MAGIC: u32 = 0x1BADB002;

pub fn multiboot_header_from_addr(addr: usize) -> &'static MultibootHeader {
    unsafe {
        &*(addr as *mut MultibootHeader)
    }
}

#[repr(C, packed)]
pub struct MultibootHeader {
    pub magic: u32,
    pub flags: u32,
    pub checksum: u32,
    pub header_addr: u32,
    pub load_addr: u32,
    pub load_end_addr: u32,
    pub bss_end_addr: u32,
    pub entry_addr: u32,
    pub mode_type: u32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

impl MultibootHeader {
    pub fn is_valid(&self) -> bool {
        self.magic == MAGIC && (self.checksum + self.flags + self.magic) == 0
    }
}
