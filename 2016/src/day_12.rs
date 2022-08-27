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
                _ => unreachable!(),
            }
        })
        .collect()
}

fn solve(input: &str, part: u8) -> i32 {
    let mut registers = HashMap::from([
        (Data::Register('a'), 0),
        (Data::Register('b'), 0),
        (Data::Register('c'), (part == 2) as i32),
        (Data::Register('d'), 0),
    ]);

    let instructions = generate(input);
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
        }
        ptr += 1;
    }
    registers[&Data::Register('a')]
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> i32 {
    solve(input, 1)
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> i32 {
    solve(input, 2)
}
