use super::data_tape::DataTape;
use super::peripheral_tape::PeripheralTape;
use super::virtual_machine_errors::VMErrKind;
use std::collections::HashMap;

pub struct BFVM<'a> {
    prog: Vec<u8>,
    jump_map: HashMap<usize, usize>,
    data_tape: DataTape,
    peripheral_tape: &'a PeripheralTape<'a>,

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
        _ => panic!(format!(
            "Unexpected non-bf char attempting to get code for: {}",
            c
        )),
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

fn create_jump_map(prog: &Vec<u8>) -> Result<HashMap<usize, usize>, VMErrKind> {
    let mut jump_map: HashMap<usize, usize> = HashMap::new();
    let mut back_stack: Vec<usize> = Vec::new();
    for (i, c) in prog.iter().enumerate() {
        if c == &BF_LOOP_OPEN {
            back_stack.push(i);
        } else if c == &BF_LOOP_CLOSE {
            if let Some(ind) = back_stack.pop() {
                jump_map.insert(i, ind);
                jump_map.insert(ind, i);
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
    //TODO: Update docs with parameters and examples
    /// Creates a new BFVM that will excecute the given augmented BrainFuck code with the given
    /// peripherals and parameters.
    pub fn new(
        code: &str,
        peripheral_tape: &'a mut PeripheralTape<'a>,
        max_pages: u32,
    ) -> Result<BFVM<'a>, VMErrKind> {
        let prog = sanitize_bf_str(code);
        let jump_map = create_jump_map(&prog)?;

        Ok(BFVM {
            prog,
            jump_map,
            data_tape: DataTape::new(max_pages),
            peripheral_tape,
            pointer: 0,
            buffer: 0,
        })
    }

    /// Runs the next character.
    pub fn next(&mut self) -> bool {
        false // TODO: implement
    }

    /// Runs `BFVM::next` for the number of times given. If `cycles` is 0, runs until program
    /// halts.
    pub fn run_for(&mut self, cycles: u32) {
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
    pub fn run(&mut self) {
        while self.next() {}
    }
}
