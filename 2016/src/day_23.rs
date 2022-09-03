use std::collections::HashMap;
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Data {
    Value(i64),
    Register(char),
}
#[derive(Debug)]
enum Instruction {
    Cpy(Data, Data),
    Inc(Data),
    Dec(Data),
    Jnz(Data, Data),
    Tgl(Data),
    // Do nothing
    Noop,
    // Adds reg1 * reg2 to reg0, sets reg2 to 0
    MulReg(Data, Data, Data),
}

fn to_data(x: Option<&str>) -> Data {
    match x.unwrap().parse() {
        Ok(i) => Data::Value(i),
        Err(_) => Data::Register(x.unwrap().chars().next().unwrap()),
    }
}

fn generate(input: &str) -> Vec<Instruction> {
    let mut x: Vec<Instruction> = input
        .lines()
        .map(|l| {
            let mut x = l.split(' ');
            match x.next().unwrap() {
                "cpy" => Instruction::Cpy(to_data(x.next()), to_data(x.next())),
                "inc" => Instruction::Inc(to_data(x.next())),
                "dec" => Instruction::Dec(to_data(x.next())),
                "jnz" => Instruction::Jnz(to_data(x.next()), to_data(x.next())),
                "tgl" => Instruction::Tgl(to_data(x.next())),
                _ => unreachable!(),
            }
        })
        .collect();

    // Replace the instructions that get executed the most with the optimized multiply function,
    // no-ops added so jumps wouldn't break
    //
    // no clue if this will work on all inputs
    *x.get_mut(4).unwrap() = Instruction::MulReg(
        Data::Register('a'),
        Data::Register('b'),
        Data::Register('d'),
    );
    for i in 5..=9 {
        *x.get_mut(i).unwrap() = Instruction::Noop;
    }
    x
}

fn solve(input: &str, part: u8) -> i64 {
    let mut registers = HashMap::from([
        (Data::Register('a'), 7 + (part == 2) as i64 * 5),
        (Data::Register('b'), 0),
        (Data::Register('c'), 0),
        (Data::Register('d'), 0),
    ]);

    let mut instructions = generate(input);
    let mut ptr = 0;
    let ilen = instructions.len();
    while ptr < ilen {
        match instructions[ptr] {
            Instruction::Cpy(x, y) => {
                if matches!(y, Data::Register(_)) {
                    registers.insert(
                        y,
                        match x {
                            Data::Value(i) => i,
                            r => registers[&r],
                        },
                    );
                }
            }
            Instruction::Inc(x) => {
                if matches!(x, Data::Register(_)) {
                    registers.entry(x).and_modify(|x| *x += 1);
                }
            }
            Instruction::Dec(x) => {
                if matches!(x, Data::Register(_)) {
                    registers.entry(x).and_modify(|x| *x -= 1);
                }
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
            Instruction::Tgl(x) => {
                let i = match x {
                    Data::Value(k) => k,
                    r => registers[&r],
                };
                if let Some(ins) = instructions.get_mut(ptr + i as usize) {
                    let new_ins = match ins {
                        Instruction::Inc(x) => Instruction::Dec(*x),
                        Instruction::Dec(x) => Instruction::Inc(*x),
                        Instruction::Tgl(x) => Instruction::Inc(*x),
                        Instruction::Jnz(a, b) => Instruction::Cpy(*a, *b),
                        Instruction::Cpy(a, b) => Instruction::Jnz(*a, *b),
                        _ => unreachable!(),
                    };
                    *ins = new_ins;
                }
            }
            Instruction::Noop => {}
            Instruction::MulReg(x, y, z) => {
                let v = registers[&y] * registers[&z];
                registers.entry(x).and_modify(|x| *x += v);
                registers.entry(z).and_modify(|z| *z = 0);
            }
        }
        ptr += 1;
    }
    registers[&Data::Register('a')]
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> i64 {
    solve(input, 1)
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> i64 {
    solve(input, 2)
}
