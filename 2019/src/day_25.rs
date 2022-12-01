use std::collections::VecDeque;

#[aoc(day25, part1)]
pub fn part1(input: &str) -> &'static str {
    let inp = std::io::stdin();
    let mut code: Vec<i64> = input.split(',').map(|x| x.parse().unwrap()).collect();
    code.append(&mut vec![0; 1000]);

    let mut c = Computer {
        code,
        ptr: 0,
        rel_offset: 0,
        input: VecDeque::new(),
        output: VecDeque::new(),
        halted: false,
    };

    while !c.halted {
        c.exec();
        if let Some(o) = c.output.pop_front() {
            print!("{}", o as u8 as char);
            if o == b'?' as i64 {
                println!();
                let mut s = String::new();
                inp.read_line(&mut s).unwrap();
                c.input.clear();
                c.input.extend(s.chars().map(|x| x as i64));
                println!();
            }
        }
    }

    "Hope you survived"
}

#[aoc(day25, part2)]
pub fn part2(_input: &str) -> &'static str {
    "Ayy"
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Computer {
    code: Vec<i64>,
    ptr: usize,
    rel_offset: i64,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    halted: bool,
}

// day 23 copied, but with panicking instead of -1 when no input

impl Computer {
    // returns if it can be executed again / it didn't halt
    pub fn exec(&mut self) {
        if self.halted {
            return;
        }
        let code = &mut self.code;
        let ptr = self.ptr;
        let mut_rel = &mut self.rel_offset;
        let rel_offset = *mut_rel;
        let input = &mut self.input;
        let op = Opcode::try_from(code[ptr]).expect("Bad opcode");
        let params = match op.instr {
            Instruction::Add | Instruction::Multiply => &code[ptr + 1..=ptr + 3],
            Instruction::Input | Instruction::Output | Instruction::RelativeBaseOffset => {
                &code[ptr + 1..=ptr + 1]
            }
            Instruction::JumpIfTrue | Instruction::JumpIfFalse => &code[ptr + 1..=ptr + 2],
            Instruction::LessThan | Instruction::Equals => &code[ptr + 1..=ptr + 3],
            Instruction::Halt => &code[ptr..=ptr], // doesn't read the params, just exits
        }
        .to_vec();

        let params: Vec<Value> = params
            .into_iter()
            .enumerate()
            // reversed
            .map(|(i, x)| match op.modes[3 - i] {
                Mode::Immediate => Value::Immediate(x),
                Mode::Position => Value::Position(x),
                Mode::Relative => Value::Relative(x),
            })
            .collect();

        let mut output = None;
        // do the operation
        match op.instr {
            Instruction::Add => {
                code[params[2].get(rel_offset) as usize] = params[0]
                    .get_val(code, rel_offset)
                    .checked_add(params[1].get_val(code, rel_offset))
                    .expect("add overflow")
            }
            Instruction::Multiply => {
                code[params[2].get(rel_offset) as usize] = params[0]
                    .get_val(code, rel_offset)
                    .checked_mul(params[1].get_val(code, rel_offset))
                    .expect("mul overflow")
            }
            Instruction::Input => {
                code[params[0].get(rel_offset) as usize] = input.pop_front().expect("No input");
            }
            Instruction::Output => output = Some(params[0].get_val(code, rel_offset)),
            Instruction::JumpIfTrue => {
                self.ptr = if params[0].get_val(code, rel_offset) != 0 {
                    params[1].get_val(code, rel_offset) as usize
                } else {
                    ptr + 3
                };
            }
            Instruction::JumpIfFalse => {
                self.ptr = if params[0].get_val(code, rel_offset) == 0 {
                    params[1].get_val(code, rel_offset) as usize
                } else {
                    ptr + 3
                };
            }
            Instruction::LessThan => {
                code[params[2].get(rel_offset) as usize] = (params[0].get_val(code, rel_offset)
                    < params[1].get_val(code, rel_offset))
                    as i64
            }
            Instruction::Equals => {
                code[params[2].get(rel_offset) as usize] = (params[0].get_val(code, rel_offset)
                    == params[1].get_val(code, rel_offset))
                    as i64
            }
            Instruction::RelativeBaseOffset => {
                *mut_rel += params[0].get_val(code, rel_offset);
            }
            Instruction::Halt => {}
        }

        match op.instr {
            Instruction::Add
            | Instruction::Multiply
            | Instruction::LessThan
            | Instruction::Equals => {
                self.ptr += 4;
            }
            Instruction::Input | Instruction::Output | Instruction::RelativeBaseOffset => {
                self.ptr += 2;
                if let Some(o) = output {
                    self.output.push_back(o);
                }
            }
            Instruction::Halt => {
                self.halted = true;
            }
            Instruction::JumpIfTrue | Instruction::JumpIfFalse => {}
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<i64> for Mode {
    type Error = &'static str;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            2 => Ok(Mode::Relative),
            _ => Err("Bad mode"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Value {
    Immediate(i64),
    Position(i64),
    Relative(i64),
}

impl Value {
    fn get_val(&self, code: &[i64], rel_off: i64) -> i64 {
        match *self {
            Value::Immediate(x) => x,
            Value::Position(x) => code[x as usize],
            Value::Relative(x) => code[(rel_off + x) as usize],
        }
    }

    fn get(&self, rel_off: i64) -> i64 {
        match self {
            Value::Position(x) | Value::Immediate(x) => *x,
            Value::Relative(x) => *x + rel_off,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[non_exhaustive] // only works at the crate level, so it doesn't do anything :/
enum Instruction {
    Add,
    Multiply,
    Input,
    Output,
    Halt,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    RelativeBaseOffset,
}

impl TryFrom<i64> for Instruction {
    type Error = &'static str;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Instruction::Add),
            2 => Ok(Instruction::Multiply),
            3 => Ok(Instruction::Input),
            4 => Ok(Instruction::Output),
            5 => Ok(Instruction::JumpIfTrue),
            6 => Ok(Instruction::JumpIfFalse),
            7 => Ok(Instruction::LessThan),
            8 => Ok(Instruction::Equals),
            9 => Ok(Instruction::RelativeBaseOffset),
            99 => Ok(Instruction::Halt),
            _ => Err("Bad instr"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Opcode {
    instr: Instruction,
    modes: [Mode; 4],
}

impl TryFrom<i64> for Opcode {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let mut value = value;
        let instr: Instruction = (value % 100).try_into()?;
        value /= 100;
        let mut modes = [Mode::Position; 4];
        for x in (0..4).rev() {
            let d = (value % 10).try_into()?;
            modes[x] = d;
            value /= 10;
        }
        Ok(Opcode { instr, modes })
    }
}
