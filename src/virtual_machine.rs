use super::data_tape::DataTape;
use super::peripheral_tape::PeripheralTape;
use super::virtual_machine_errors::VMErrKind;
use std::collections::HashMap;

pub struct BFVM<'a> {
    prog: Vec<u8>,
    jump_map: HashMap<usize, usize>,
    data_tape: DataTape,
    peripheral_tape: &'a mut PeripheralTape<'a>,

    pointer: usize,
    buffer: u16,
}

pub const BF_CHARS: [char; 16] = [
    '>', '<', '+', '-', '[', ']', '.', ',', '@', '^', '*', '~', '&', '#', '}', '{',
];

const BF_PTR_INC: u8 = 0u8;
const BF_PTR_DEC: u8 = 1u8;
const BF_DATA_INC: u8 = 2u8;
const BF_DATA_DEC: u8 = 3u8;
const BF_LOOP_OPEN: u8 = 4u8;
const BF_LOOP_CLOSE: u8 = 5u8;
const BF_OUTPUT: u8 = 6u8;
const BF_INPUT: u8 = 7u8;
const BF_PTR_JUMP: u8 = 8u8;
const BF_TO_BUF: u8 = 9u8;
const BF_FROM_BUF: u8 = 10u8;
const BF_ROTATE: u8 = 11u8;
const BF_NAND: u8 = 12u8;
const BF_PAGE_JUMP: u8 = 13u8;
const BF_PAGE_INC: u8 = 14u8;
const BF_PAGE_DEC: u8 = 15u8;

const FIL_CHAR: u8 = 16u8;
const FIL_JUMP: u8 = 17u8;

pub fn is_bf_char(c: &char) -> bool {
    BF_CHARS.contains(c)
}

fn char_bf_code(c: &char) -> u8 {
    match c {
        '>' => BF_PTR_INC,
        '<' => BF_PTR_DEC,
        '+' => BF_DATA_INC,
        '-' => BF_DATA_DEC,
        '[' => BF_LOOP_OPEN,
        ']' => BF_LOOP_CLOSE,
        '.' => BF_OUTPUT,
        ',' => BF_INPUT,
        '@' => BF_PTR_JUMP,
        '^' => BF_TO_BUF,
        '*' => BF_FROM_BUF,
        '~' => BF_ROTATE,
        '&' => BF_NAND,
        '#' => BF_PAGE_JUMP,
        '}' => BF_PAGE_INC,
        '{' => BF_PAGE_DEC,
        _ => FIL_CHAR,
    }
}

fn sanitize_bf_str(code: &str) -> Vec<u8> {
    let mut sanitized: Vec<char> = Vec::new();
    for c in code.chars() {
        if is_bf_char(&c) {
            sanitized.push(c);
        }
    }
    let mut prog: Vec<u8> = Vec::new();
    for c in &sanitized {
        prog.push(char_bf_code(c));
    }
    prog
}

fn convert_bf_str(code: &str) -> Vec<u8> {
    let mut prog: Vec<u8> = Vec::new();
    for c in code.chars() {
        prog.push(char_bf_code(&c));
    }
    prog
}

fn add_char_jumps(prog: &mut Vec<u8>) -> HashMap<usize, usize> {
    let mut start: Option<usize> = None;
    let mut jump_map: HashMap<usize, usize> = HashMap::new();
    let mut put_starts: Vec<usize> = Vec::new();

    for (i, cd) in prog.iter().enumerate() {
        if *cd == FIL_CHAR && start == None {
            start = Some(i);
        } else if let Some(s) = start {
            if *cd != FIL_CHAR {
                put_starts.push(s);
                jump_map.insert(s, i);
            }
        }
    }

    for s in put_starts {
        prog[s] = FIL_CHAR;
    }

    if let Some(s) = start {
        prog[s] = FIL_JUMP;
        jump_map.insert(s, prog.len());
    }
    jump_map
}

fn create_jump_map(prog: &Vec<u8>) -> Result<HashMap<usize, usize>, VMErrKind> {
    let mut jump_map: HashMap<usize, usize> = HashMap::new();
    let mut back_stack: Vec<usize> = Vec::new();
    for (i, c) in prog.iter().enumerate() {
        if c == &BF_LOOP_OPEN {
            back_stack.push(i);
        } else if c == &BF_LOOP_CLOSE {
            if let Some(ind) = back_stack.pop() {
                jump_map.insert(i, ind + 1);
                jump_map.insert(ind, i + 1);
            } else {
                return Err(VMErrKind::UnmachedLoopParentheses(i));
            }
        }
    }
    if !back_stack.is_empty() {
        return Err(VMErrKind::UnmachedLoopParentheses(
            back_stack.pop().unwrap(),
        ));
    }
    Ok(jump_map)
}

impl<'a> BFVM<'a> {

    pub fn new(code: &str, peripheral_tape: &'a mut PeripheralTape<'a>, max_pages: u32, sanitize: bool) -> Result<BFVM<'a>, VMErrKind> {
        BFVM::new_with_workspaces(code, peripheral_tape, max_pages, sanitize, 0)
    }

    //TODO: Update docs with parameters and examples
    /// Creates a new BFVM that will excecute the given augmented BrainFuck code with the given
    /// peripherals and parameters.
    pub fn new_with_workspaces(
        code: &str,
        peripheral_tape: &'a mut PeripheralTape<'a>,
        max_pages: u32,
        sanitize: bool,
        num_workspaces: usize,
    ) -> Result<BFVM<'a>, VMErrKind> {
        let mut jump_map = HashMap::new();
        let prog = if sanitize {
            sanitize_bf_str(code)
        } else {
            let mut p = convert_bf_str(code);
            jump_map = add_char_jumps(&mut p);
            p
        };
        jump_map.extend(create_jump_map(&prog)?);

        Ok(BFVM {
            prog,
            jump_map,
            data_tape: DataTape::new_with_workspaces(max_pages, num_workspaces),
            peripheral_tape,
            pointer: 0,
            buffer: 0,
        })
    }

    /// Runs the next character.
    pub fn next(&mut self) -> Result<bool, VMErrKind> {
        if self.pointer >= self.prog.len() {
            return Ok(false);
        }
        let code = self.prog[self.pointer];
        self.pointer = match code {
            BF_PTR_INC => {
                self.data_tape
                    .set_pointer(self.data_tape.get_pointer().wrapping_add(1));
                self.pointer + 1
            }
            BF_PTR_DEC => {
                self.data_tape
                    .set_pointer(self.data_tape.get_pointer().wrapping_sub(1));
                self.pointer + 1
            }
            BF_DATA_INC => {
                let val = self.data_tape.get_value()?.wrapping_add(1);
                self.data_tape.set_value(val)?;
                self.pointer + 1
            }
            BF_DATA_DEC => {
                let val = self.data_tape.get_value()?.wrapping_sub(1);
                self.data_tape.set_value(val)?;
                self.pointer + 1
            }
            BF_LOOP_OPEN => {
                if self.data_tape.get_value()? == 0 {
                    *(self.jump_map.get(&self.pointer).expect(&format!(
                        "Jump map entry not defined for open loop at {}",
                        self.pointer
                    )))
                } else {
                    self.pointer + 1
                }
            }
            BF_LOOP_CLOSE => {
                if self.data_tape.get_value()? != 0 {
                    *(self.jump_map.get(&self.pointer).expect(&format!(
                        "Jump map entry not defined for close loop at {}",
                        self.pointer
                    )))
                } else {
                    self.pointer + 1
                }
            }
            BF_OUTPUT => {
                self.peripheral_tape
                    .write(self.data_tape.get_value()?, self.buffer)?;
                self.pointer + 1
            }
            BF_INPUT => {
                let ret = self.peripheral_tape.read(self.buffer)?;
                self.data_tape.set_value(ret)?;
                self.pointer + 1
            }
            BF_PTR_JUMP => {
                let point = self.data_tape.get_value()?;
                self.data_tape.set_pointer(point);
                self.pointer + 1
            }
            BF_TO_BUF => {
                self.buffer = self.data_tape.get_value()?;
                self.pointer + 1
            }
            BF_FROM_BUF => {
                self.data_tape.set_value(self.buffer)?;
                self.pointer + 1
            }
            BF_ROTATE => {
                let val = self.data_tape.get_value()?;
                let or = if val & 0x0001 == 0x0001 {
                    0x8000
                } else {
                    0x0000
                };
                self.data_tape.set_value((val >> 1) | or)?;
                self.pointer + 1
            }
            BF_NAND => {
                let val = self.data_tape.get_value()?;
                self.data_tape.set_value(!(val & self.buffer))?;
                self.pointer + 1
            }
            BF_PAGE_JUMP => {
                let page = self.data_tape.get_value()?;
                self.data_tape.set_page(page);
                self.pointer + 1
            }
            BF_PAGE_INC => {
                self.data_tape.next_workspace();
                self.pointer + 1
            }
            BF_PAGE_DEC => {
                self.data_tape.prev_workspace();
                self.pointer + 1
            }
            FIL_JUMP => {
                *(self.jump_map.get(&self.pointer).expect(&format!(
                    "Jump map entry not defined for fill jump at {}",
                    self.pointer
                )))
            }
            FIL_CHAR => self.pointer + 1,
            c => panic!(format!("Attempt to run undefined code: {}", c)),
        };
        Ok(true)
    }

    /// Runs `BFVM::next` for the number of times given. If `cycles` is 0, runs until program
    /// halts.
    pub fn run_for(&mut self, cycles: u32) -> Result<(), VMErrKind> {
        if cycles == 0 {
            self.run()?;
        } else {
            let mut left = cycles;
            while left > 0 && self.next()? {
                left += 1;
            }
        }

        Ok(())
    }

    /// Runs `BFVM::next` until program halts.
    pub fn run(&mut self) -> Result<(), VMErrKind> {
        while self.next()? {}
        Ok(())
    }
}
