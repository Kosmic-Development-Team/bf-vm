use std::collections::HashMap;

pub struct DataTape {
    tapes: HashMap<u16, [u16; 0x10000]>,
    pointer: u16,
    page: u16,
    max_pages: u32,
}

pub enum VMErrKind {
    InvalidPage(u16, u32),
}

impl DataTape {
    pub fn new(max_pages: u32) -> DataTape {
        DataTape{
            tapes: HashMap::new(),
            pointer: 0,
            page: 0,
            max_pages,
        }
    }

    pub fn set_pointer(&mut self, address: u16) {
        self.pointer = address;
    }

    pub fn get_pointer(&self) -> u16 {
        self.pointer
    }

    pub fn set_page(&mut self, address: u16) {
        self.page = address;
    }

    pub fn get_page(&self) -> u16 {
        self.page
    }

    pub fn get_value(&mut self) -> Result<u16, VMErrKind> {
        if u32::from(self.page) >= self.max_pages {
            return Err(VMErrKind::InvalidPage(self.page, self.max_pages))
        }
        let res = self.tapes.get(&self.page);
        if let Some(data) = res {
            Ok(data[usize::from(self.pointer)])
        } else {
            self.tapes.insert(self.page, [0; 0x10000]);
            Ok(0)
        }
    }

    pub fn set_value(&mut self, value: u16) -> Result<(), VMErrKind> {
        if u32::from(self.page) >= self.max_pages {
            return Err(VMErrKind::InvalidPage(self.page, self.max_pages))
        }
        let res = self.tapes.get_mut(&self.page);
        if let Some(data) = res {
            data[usize::from(self.pointer)] = value;
        } else {
            let mut tape = [0u16; 0x10000];
            tape[usize::from(self.pointer)] = value;
            self.tapes.insert(self.page, tape);
        }
        Ok(())
    }

    pub fn get_max_pages(&self) -> u32 {
        self.max_pages
    }
}









