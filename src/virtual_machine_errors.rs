pub enum VMErrKind {
   InvalidPage(u16, u32),                       // page, max pages
   PeripheralIOErr(u16, u16),                   // value, address
   OverlappingPeripheralAddresses(usize, u32),  // peripheral vector length, smallest native address
   InvalidPeripheralTapeAccess(u16),            // peripheral tape address
   UnmachedLoopParentheses(usize),              // unmached parenthesis location
}


