enum Reg {
    A,
    B,
}

enum Command {
    Hlf(Reg),
    Tpl(Reg),
    Inc(Reg),
    Jmp(i32),
    Jie(Reg, i32),
    Jio(Reg, i32),
}

fn parse_command(line: &str) -> Command {
    let (command, rest) = line.split_at(3);
    let rest = rest.trim();
    match command {
        "hlf" => match rest {
            "a" => Command::Hlf(Reg::A),
            _ => Command::Hlf(Reg::B),
        },
        "tpl" => match rest {
            "a" => Command::Tpl(Reg::A),
            _ => Command::Tpl(Reg::B),
        },
        "inc" => match rest {
            "a" => Command::Inc(Reg::A),
            _ => Command::Inc(Reg::B),
        },
        "jmp" => Command::Jmp(rest.parse().unwrap()),
        "jie" => {
            let (reg, val) = rest.split_at(2);
            let val = val.trim().parse::<i32>().unwrap();
            match reg {
                "a," => Command::Jie(Reg::A, val),
                _ => Command::Jie(Reg::B, val),
            }
        }
        "jio" => {
            let (reg, val) = rest.split_at(2);
            let val = val.trim().parse::<i32>().unwrap();
            match reg {
                "a," => Command::Jio(Reg::A, val),
                _ => Command::Jio(Reg::B, val),
            }
        }
        _ => unreachable!(),
    }
}

fn solve(input: &str, a: i32, b: i32) -> i32 {
    let commands: Vec<Command> = input.lines().map(parse_command).collect();
    let mut a = a;
    let mut b = b;
    let mut ptr: i32 = 0;
    while ptr < commands.len() as i32 {
        let command = &commands[ptr as usize];
        match command {
            Command::Hlf(reg) => {
                match reg {
                    Reg::A => a /= 2,
                    Reg::B => b /= 2,
                };
                ptr += 1;
            }
            Command::Tpl(reg) => {
                match reg {
                    Reg::A => a *= 3,
                    Reg::B => b *= 3,
                };
                ptr += 1;
            }
            Command::Inc(reg) => {
                match reg {
                    Reg::A => a += 1,
                    Reg::B => b += 1,
                };
                ptr += 1;
            }
            Command::Jmp(x) => ptr += *x,
            Command::Jie(reg, x) => match reg {
                Reg::A => {
                    if a % 2 == 0 {
                        ptr += x;
                    } else {
                        ptr += 1;
                    }
                }
                Reg::B => {
                    if b % 2 == 0 {
                        ptr += x;
                    } else {
                        ptr += 1;
                    }
                }
            },
            Command::Jio(reg, x) => match reg {
                Reg::A => {
                    if a == 1 {
                        ptr += x;
                    } else {
                        ptr += 1;
                    }
                }
                Reg::B => {
                    if b == 1 {
                        ptr += x;
                    } else {
                        ptr += 1;
                    }
                }
            },
        }
    }
    b
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> i32 {
    solve(input, 0, 0)
}

// One line is different
#[aoc(day23, part2)]
pub fn part2(input: &str) -> i32 {
    solve(input, 1, 0)
}
