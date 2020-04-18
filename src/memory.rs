use bit_field::BitField;
use core::fmt;

#[repr(transparent)]
pub struct PageTableEntry {
    entry: u64
}

impl PageTableEntry {
    pub fn from(entry: u64) -> PageTableEntry {
        PageTableEntry { entry }
    }
    pub fn as_u64(&self) -> u64 {
        self.entry as u64
    }

    pub fn is_present(&self) -> bool {
        self.entry.get_bit(0)
    }
    pub fn is_writeable(&self) -> bool { 
        self.entry.get_bit(1)
    }
    pub fn is_user_accessible(&self) -> bool { 
        self.entry.get_bit(2)
    }
    pub fn is_write_through_caching(&self) -> bool { 
        self.entry.get_bit(3)
    }
    pub fn is_disable_cache(&self) -> bool { 
        self.entry.get_bit(4)
    }
    pub fn is_accessed(&self) -> bool { 
        self.entry.get_bit(5)
    }
    pub fn is_dirty(&self) -> bool { 
        self.entry.get_bit(6)
    }
    pub fn is_huge_page(&self) -> bool { 
        self.entry.get_bit(7)
    }
    pub fn is_global(&self) -> bool { 
        self.entry.get_bit(8)
    }
    pub fn is_not_executable(&self) -> bool {
        self.entry.get_bit(63)
    }
    pub fn physical_address(&self) -> u64 {
        self.entry.get_bits(12..=51) << 12
    }

    pub fn is_continuation_of(&self, previous: &PageTableEntry) -> bool {
        if self.is_present() != previous.is_present() ||
            self.is_writeable() != previous.is_writeable() ||
            self.is_user_accessible() != previous.is_user_accessible() ||
            self.is_write_through_caching() != previous.is_write_through_caching() ||
            self.is_disable_cache() != previous.is_disable_cache() ||
            self.is_accessed() != previous.is_accessed() ||
            self.is_dirty() != previous.is_dirty() ||
            self.is_huge_page() != previous.is_huge_page() ||
            self.is_global() != previous.is_global() ||
            self.is_not_executable() != previous.is_not_executable() { 
                return false; 
        }
        if self.is_huge_page() {
            return self.physical_address() == previous.physical_address() + 0x200000;
        }
        else {
            return self.physical_address() == previous.physical_address() + 0x1000;
        }
    }

    
}
impl fmt::Debug for PageTableEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PageTableEntry(PhysAddr=0x{:x}, present={}, writeable={}, user_accessible={}, dirty={}, accessed={}, nx={}, huge_page={})", 
            self.physical_address(),
            self.is_present(),
            self.is_writeable(),
            self.is_user_accessible(),
            self.is_dirty(),
            self.is_accessed(),
            self.is_not_executable(),
            self.is_huge_page()
        )
    }
}


pub fn describe_page_table(virtual_address: u64, physical_address: u64, level: u64, memory_mapping_offset: u64) {
    
    let virtual_address_offset = 1 << (12 + 9 * (level-1));
    
    
    let ptr = (physical_address + memory_mapping_offset) as *const [u64; 512];
    let page_table = unsafe { &(*ptr) };
    
    let mut previous_entry = 0;
    let mut nb_continuation = 0;
    for ix in 0..512 {
        let page_table_entry = page_table[ix as usize];
        nb_continuation = describe_page_table_entry(
            virtual_address + ix * virtual_address_offset, 
            physical_address + ix * 8, 
            page_table_entry,
            ix, 
            level, 
            memory_mapping_offset, 
            previous_entry, 
            nb_continuation);
        previous_entry = page_table_entry;
    }
}

pub fn describe_page_table_entry(virtual_address: u64, physical_address: u64, page_table_entry_u64: u64, index: u64, level: u64, memory_mapping_offset: u64, previous_entry: u64, nb_continuation: u32 ) -> u32 {
    use crate::serial_println;

    let indent = match level {
        4 => "",
        3 => "  ",
        2 => "    ",
        1 => "      ",
        _ => panic!("Error! level should be 1..=4")
    };

    let page_table_entry = PageTableEntry::from(page_table_entry_u64);

    if !page_table_entry.is_present() { return 0; }

    if index > 0 && index < 511 {
        let previous_page_table_entry = PageTableEntry::from(previous_entry);
        if page_table_entry.is_continuation_of(&previous_page_table_entry) {
            return nb_continuation + 1;
        }
    }

    // we are not a continuation, or the last entry in the table, so we display the page table entry
    if nb_continuation > 0 {
        serial_println!("                            ... + {} ... ", nb_continuation);
    }

    serial_println!("{} {} {:3} 0x{:x}... {:?}                             (stored in physical address 0x{:x})", level, indent, index, virtual_address, page_table_entry, physical_address);

    if !page_table_entry.is_huge_page() && level > 1 {
        describe_page_table(virtual_address, page_table_entry.physical_address(), level - 1, memory_mapping_offset);
    }

    return 0;
    
}