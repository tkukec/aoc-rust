use std::collections::HashMap;
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Data {
    Value(i32),
    Register(char),
}
#[derive(Debug)]
enum Instruction {
    Cpy(Data, Data),
    Inc(Data),
    Dec(Data),
    Jnz(Data, Data),
    Out(Data),
}

fn to_data(x: Option<&str>) -> Data {
    match x.unwrap().parse() {
        Ok(i) => Data::Value(i),
        Err(_) => Data::Register(x.unwrap().chars().next().unwrap()),
    }
}

fn generate(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let mut x = l.split(' ');
            match x.next().unwrap() {
                "cpy" => Instruction::Cpy(to_data(x.next()), to_data(x.next())),
                "inc" => Instruction::Inc(to_data(x.next())),
                "dec" => Instruction::Dec(to_data(x.next())),
                "jnz" => Instruction::Jnz(to_data(x.next()), to_data(x.next())),
                "out" => Instruction::Out(to_data(x.next())),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn gives_clock(instructions: &Vec<Instruction>, a_val: i32) -> bool {
    let mut clock_history = vec![];
    let mut registers = HashMap::from([
        (Data::Register('a'), a_val),
        (Data::Register('b'), 0),
        (Data::Register('c'), 0),
        (Data::Register('d'), 0),
    ]);

    let mut ptr = 0;
    let ilen = instructions.len();
    while ptr < ilen {
        match instructions[ptr] {
            Instruction::Cpy(x, y) => {
                registers.insert(
                    y,
                    match x {
                        Data::Value(i) => i,
                        r => registers[&r],
                    },
                );
            }
            Instruction::Inc(x) => {
                registers.entry(x).and_modify(|x| *x += 1);
            }
            Instruction::Dec(x) => {
                registers.entry(x).and_modify(|x| *x -= 1);
            }
            Instruction::Jnz(x, y) => {
                if match x {
                    Data::Value(i) => i,
                    r => registers[&r],
                } != 0
                {
                    ptr += match y {
                        Data::Value(i) => i,
                        r => registers[&r],
                    } as usize
                        - 1;
                }
            }
            Instruction::Out(x) => {
                let val = match x {
                    Data::Value(i) => i,
                    r => registers[&r],
                };
                if !(val == 0 || val == 1 && !clock_history.is_empty()) {
                    return false;
                } else {
                    if clock_history.contains(&(val, registers.clone()))
                        && clock_history.last().unwrap().0 != val
                    {
                        return true;
                    }
                    if clock_history.last().unwrap_or(&(2, HashMap::new())).0 == val {
                        return false;
                    }
                    clock_history.push((val, registers.clone()))
                }
            }
        }
        ptr += 1;
    }
    unreachable!()
}

#[aoc(day25, part1)]
pub fn part1(input: &str) -> i32 {
    let instructions = generate(input);
    for x in 0.. {
        if gives_clock(&instructions, x) {
            return x;
        }
    }
    unreachable!()
}

#[aoc(day25, part2)]
pub fn part2(_input: &str) -> String {
    "gg".to_owned()
}
