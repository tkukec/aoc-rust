use std::collections::{HashSet, VecDeque};

#[aoc(day23, part1)]
pub fn part1(input: &str) -> i64 {
    let mut code: Vec<i64> = input.split(',').map(|x| x.parse().unwrap()).collect();
    code.append(&mut vec![0; 100]);
    let mut computers = vec![
        Computer {
            code,
            ptr: 0,
            rel_offset: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            halted: false,
        };
        50
    ];
    for (i, c) in computers.iter_mut().enumerate() {
        c.input.push_back(i as i64);
    }
    while computers.iter().any(|x| !x.halted) {
        // can't do iter_mut() because of borrow rules
        for n in 0..computers.len() {
            let i = &mut computers[n];
            i.exec();
            if i.output.len() >= 3 {
                let reciever = i.output.pop_front().unwrap();
                let x = i.output.pop_front().unwrap();
                let y = i.output.pop_front().unwrap();
                if reciever == 255 {
                    return y;
                }
                computers[reciever as usize].input.push_back(x);
                computers[reciever as usize].input.push_back(y);
            }
        }
    }

    unreachable!()
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> i64 {
    let mut code: Vec<i64> = input.split(',').map(|x| x.parse().unwrap()).collect();
    code.append(&mut vec![0; 100]);
    let mut computers = vec![
        Computer {
            code,
            ptr: 0,
            rel_offset: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            halted: false,
        };
        50
    ];
    for (i, c) in computers.iter_mut().enumerate() {
        c.input.push_back(i as i64);
    }
    let mut last_packet = (i64::MIN, i64::MIN);
    let mut last_packet_timer = 0;
    let mut seen_values = HashSet::new();
    while computers.iter().any(|x| !x.halted) {
        last_packet_timer += 1;
        let mut packets_sent = false;
        for n in 0..computers.len() {
            let i = &mut computers[n];
            i.exec();
            if i.output.len() >= 3 {
                let reciever = i.output.pop_front().unwrap();
                let x = i.output.pop_front().unwrap();
                let y = i.output.pop_front().unwrap();
                packets_sent = true;
                if reciever == 255 {
                    last_packet = (x, y);
                    continue;
                }
                computers[reciever as usize].input.push_back(x);
                computers[reciever as usize].input.push_back(y);
            }
        }
        if packets_sent {
            last_packet_timer = 0;
        }
        if last_packet_timer > 1000 {
            computers[0].input.push_back(last_packet.0);
            computers[0].input.push_back(last_packet.1);
            last_packet_timer = 0;
            if seen_values.contains(&last_packet.1) {
                return last_packet.1;
            } else {
                seen_values.insert(last_packet.1);
            }
        }
    }

    unreachable!()
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

// local copy of intcode to make modifications easier

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
                code[params[0].get(rel_offset) as usize] = input.pop_front().unwrap_or(-1);
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
