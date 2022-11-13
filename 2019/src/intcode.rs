use std::collections::VecDeque;

// returns new ptr (None if halted) and output (None if not opcode 4)
pub fn exec_at(
    code: &mut [i64],
    ptr: usize,
    input: &mut VecDeque<i64>,
    rel_offset: &mut i64,
) -> (Option<usize>, Option<i64>) {
    let mut_rel = rel_offset;
    let rel_offset = *mut_rel;
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
            code[params[0].get(rel_offset) as usize] =
                input.pop_front().expect("Input not long enough")
        }
        Instruction::Output => output = Some(params[0].get_val(code, rel_offset)),
        Instruction::JumpIfTrue => {
            return (
                Some(if params[0].get_val(code, rel_offset) != 0 {
                    params[1].get_val(code, rel_offset) as usize
                } else {
                    ptr + 3
                }),
                None,
            );
        }
        Instruction::JumpIfFalse => {
            return (
                Some(if params[0].get_val(code, rel_offset) == 0 {
                    params[1].get_val(code, rel_offset) as usize
                } else {
                    ptr + 3
                }),
                None,
            );
        }
        Instruction::LessThan => {
            code[params[2].get(rel_offset) as usize] =
                (params[0].get_val(code, rel_offset) < params[1].get_val(code, rel_offset)) as i64
        }
        Instruction::Equals => {
            code[params[2].get(rel_offset) as usize] =
                (params[0].get_val(code, rel_offset) == params[1].get_val(code, rel_offset)) as i64
        }
        Instruction::RelativeBaseOffset => {
            *mut_rel += params[0].get_val(code, rel_offset);
        }
        Instruction::Halt => {}
    }

    match op.instr {
        Instruction::Add | Instruction::Multiply | Instruction::LessThan | Instruction::Equals => {
            (Some(ptr + 4), None)
        }
        Instruction::Input | Instruction::Output | Instruction::RelativeBaseOffset => {
            (Some(ptr + 2), output)
        }
        Instruction::Halt => (None, None),
        Instruction::JumpIfTrue | Instruction::JumpIfFalse => unreachable!(),
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
