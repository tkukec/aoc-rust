#[derive(Debug)]
enum Ins {
    SwapInd(u8, u8),
    SwapLet(char, char),
    RotateL(u8),
    RotateR(u8),
    RotateByInd(char),
    ReverseRange(u8, u8),
    Move(u8, u8),
}
fn generate(input: &str) -> Vec<Ins> {
    let mut instr: Vec<Ins> = vec![];
    for x in input.lines() {
        instr.push(match &x[..8] {
            "swap pos" => {
                let a = x.chars().nth(14).unwrap().to_digit(10).unwrap();
                let b = x.chars().last().unwrap().to_digit(10).unwrap();
                Ins::SwapInd(a as u8, b as u8)
            }
            "swap let" => {
                let a = x.chars().nth(12).unwrap();
                let b = x.chars().last().unwrap();
                Ins::SwapLet(a, b)
            }
            "rotate l" => {
                let a = x.chars().nth(12).unwrap().to_digit(10).unwrap();
                Ins::RotateL(a as u8)
            }
            "rotate r" => {
                let a = x.chars().nth(13).unwrap().to_digit(10).unwrap();
                Ins::RotateR(a as u8)
            }
            "rotate b" => {
                let a = x.chars().last().unwrap();
                Ins::RotateByInd(a)
            }
            "reverse " => {
                let a = x.chars().nth(18).unwrap().to_digit(10).unwrap();
                let b = x.chars().last().unwrap().to_digit(10).unwrap();
                Ins::ReverseRange(a as u8, b as u8)
            }
            "move pos" => {
                let a = x.chars().nth(14).unwrap().to_digit(10).unwrap();
                let b = x.chars().last().unwrap().to_digit(10).unwrap();
                Ins::Move(a as u8, b as u8)
            }
            _ => unreachable!(),
        })
    }

    instr
}
#[aoc(day21, part1)]
pub fn part1(input: &str) -> String {
    let instr = generate(input);
    let mut s: Vec<char> = "abcdefgh".chars().collect();
    for x in instr {
        match x {
            Ins::SwapInd(a, b) => {
                s.swap(a as usize, b as usize);
            }
            Ins::SwapLet(a, b) => s.iter_mut().for_each(|x| {
                if *x == a {
                    *x = b
                } else if *x == b {
                    *x = a
                }
            }),
            Ins::RotateL(a) => s.rotate_left(a as usize),
            Ins::RotateR(a) => s.rotate_right(a as usize),
            Ins::RotateByInd(a) => {
                let a = s.iter().position(|x| *x == a).unwrap();
                s.rotate_right((1 + a + (a >= 4) as usize) % 8)
            }
            Ins::ReverseRange(a, b) => {
                s.get_mut((a as usize)..=(b as usize)).unwrap().reverse();
            }
            Ins::Move(a, b) => {
                let x = s.remove(a as usize);
                s.insert(b as usize, x);
            }
        }
    }
    String::from_iter(s)
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> String {
    let mut instr = generate(input);
    instr.reverse();
    let mut s: Vec<char> = "fbgdceah".chars().collect();
    for x in instr {
        match x {
            Ins::SwapInd(a, b) => {
                s.swap(a as usize, b as usize);
            }
            Ins::SwapLet(a, b) => s.iter_mut().for_each(|x| {
                if *x == a {
                    *x = b
                } else if *x == b {
                    *x = a
                }
            }),
            Ins::RotateL(a) => s.rotate_right(a as usize),
            Ins::RotateR(a) => s.rotate_left(a as usize),
            Ins::RotateByInd(a) => {
                let a = s.iter().position(|x| *x == a).unwrap();
                // rotate based on position of char A
                // x = index of A
                // x + 1 + x >= 4
                // x f(x) new_x
                // 0  1  | 1
                // 1  2  | 3
                // 2  3  | 5
                // 3  4  | 7
                // 4  6  | 2
                // 5  7  | 4
                // 6  0  | 6
                // 7  1  | 0
                //
                // if A ends up on 0, before that it was on 7, and was moved by 1
                // if A ends up on 1, before that it was on 0, and was moved by 1,
                // if A ends up on 2, before that it was on 4, and was moved by 6,
                // ...
                match a {
                    0 | 1 => s.rotate_left(1),
                    2 => s.rotate_left(6),
                    3 => s.rotate_left(2),
                    4 => s.rotate_left(7),
                    5 => s.rotate_left(3),
                    6 => {}
                    7 => s.rotate_left(4),
                    _ => unreachable!(),
                }
            }
            Ins::ReverseRange(a, b) => {
                s.get_mut((a as usize)..=(b as usize)).unwrap().reverse();
            }
            Ins::Move(a, b) => {
                let x = s.remove(b as usize);
                s.insert(a as usize, x);
            }
        }
    }
    String::from_iter(s)
}
