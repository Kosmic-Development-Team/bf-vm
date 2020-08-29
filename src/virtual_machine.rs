pub struct BFVM {
    // TODO: necessary running variables
}

impl BFVM {

    /// Creates a new BFVM that will excecute the given augmented BrainFuck code with the given
    /// peripherals and parameters.
    //pub fn new(/* code, peripherals and other things */) -> &BFVM {
        
    //}

    /// Runs the next character.
    pub fn next(&self) -> bool {
        false // TODO: implement
    }

    /// Runs `BFVM::next` for the number of times given. If `cycles` is 0, runs until program
    /// halts. 
    pub fn run_for(&self, cycles: u32) {
        if cycles == 0 {
            self.run();
        } else {
            let mut left = cycles;
            while left > 0 && self.next() {
                left += 1;
            }
        }
    }

    /// Runs `BFVM::next` until program halts.
    pub fn run(&self) {
        while self.next() {}
    }
}
