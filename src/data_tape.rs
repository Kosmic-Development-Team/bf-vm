use std::collections::HashMap;
use super::virtual_machine_errors::VMErrKind;

/// A paginated data tape.
pub struct DataTape {
    tapes: HashMap<u16, [u16; 0x10000]>,
    pointer: u16,
    page: u16,
    max_pages: u32,
}

impl DataTape {

    //TODO: bounds on max pages
    /// Constructs a new, empty `DataTape`.
    /// The tape will create pages when read from or written to the first time.
    /// # Examples
    /// ```
    /// let mut tape: DataTape = DataTape::new(0x10000);
    /// ```
    pub fn new(max_pages: u32) -> DataTape {
        DataTape{
            tapes: HashMap::new(),
            pointer: 0,
            page: 0,
            max_pages,
        }
    }

    /// Set the address of the tape data pointer.
    /// # Examples
    /// ```
    /// let mut tape = DataTape::new(0x10000);
    /// tape.set_pointer(0xBEEF);
    /// ```
    pub fn set_pointer(&mut self, address: u16) {
        self.pointer = address;
    }

    /// Get the address of the tape data pointer.
    /// # Examples
    /// ```
    /// let mut tape = DataTape::new(0x10000);
    /// tape.set_pointer(0xBEEF);
    /// assert_eq!(tape.get_pointer(), 0xBEEF);
    /// ```
    pub fn get_pointer(&self) -> u16 {
        self.pointer
    }

    /// Set the current memory page.
    /// # Examples
    /// ```
    /// let mut tape = DataTape::new(0x10000);
    /// tape.set_page(10u16);
    ///
    /// ```
    pub fn set_page(&mut self, address: u16) {
        self.page = address;
    }

    /// Get the current memory page.
    /// # Examples
    /// ```
    /// let mut tape = DataTape::new(0x10000);
    /// tape.set_page(10u16);
    /// assert_eq!(tape.get_page(), 10u16);
    ///
    /// ```
    pub fn get_page(&self) -> u16 {
        self.page
    }

    /// Set the value at the pointer on the tape.
    /// # Errors
    /// If the page is out of bounds, then a `VMErrKind::InvalidPage` error is returned.
    /// # Examples
    /// ```
    /// let mut tape = DataTape::new(0x10000);
    /// tape.set_value(0xDEAD);
    /// ```
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

    /// Get the value at the pointer on the tape.
    /// # Errors
    /// If the page is out of bounds, then a `VMErrKind::InvalidPage` error is returned.
    /// # Examples
    /// ```
    /// let mut tape = DataTape::new(0x10000);
    /// tape.set_value(0xDEAD);
    /// assert_eq!(tape.get_value(), 0xDEAD);
    /// ```
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

    /// Gets the maximum number of accessible pages.
    /// # Examples
    /// ```
    /// let mut tape = DataTape::new(42u16);
    /// assert_eq!(tape.get_max_pages, 42u16);
    /// ```
    pub fn get_max_pages(&self) -> u32 {
        self.max_pages
    }
}









