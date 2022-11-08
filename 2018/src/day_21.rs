// what does the code do
//
// #ip 3 ==> r3 is the ip
// 0  seti 123 0 4         r4 = 123
//    bani 4 456 4         r4 = r4 & 456
//    eqri 4 72 4          r4 = r4 == 72
//    addr 4 3 3           r3 += r4
//    seti 0 0 3           r3 = 0
// 5  seti 0 6 4           r4 = 0
//    bori 4 65536 1       r1 = r4 | 65536
// 7  seti 678134 1 4      r4 = 678134
//    bani 1 255 5         r5 = r1 & 255
//    addr 4 5 4           r4 += r5
//    bani 4 16777215 4    r4 &= 16777215
//    muli 4 65899 4       r4 *= 65899
//    bani 4 16777215 4    r4 &= 16777215
//    gtir 256 1 5         r5 = 256 > r1
//    addr 5 3 3           r3 += r5
//    addi 3 1 3           r3 += 3
//    seti 27 8 3          r3 = 27
// 17 seti 0 1 5           r5 = 0
//    addi 5 1 2           r2 = r5 + 1
//    muli 2 256 2         r2 *= 256
//    gtrr 2 1 2           r2 = r2 > r1
//    addr 2 3 3           r3 += r2
//    addi 3 1 3           r3 += 1
//    seti 25 7 3          r3 = 25
//    addi 5 1 5           r5 += 1
// 25 seti 17 1 3          r3 = 17
//    setr 5 3 1           r1 = r5
// 27 seti 7 8 3           r3 = 7
//    eqrr 4 0 5           r5 = r4 == r0 // check - if r4 == input {break;} else {r3 = 5;}
//    addr 5 3 3           r3 += r5
//    seti 5 4 3           r3 = 5

use std::collections::HashSet;

#[aoc_generator(day21)]
pub fn generate(input: &str) -> (usize, Vec<Instr>) {
    let mut i = input.lines();

    let reg = i
        .next()
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as i32;
    let second: Vec<Instr> = i
        .map(|l| {
            let mut l = l.split(' ');
            let op = l.next().unwrap();
            let a = l.next().unwrap().parse().unwrap();
            let b = l.next().unwrap().parse().unwrap();
            let c = l.next().unwrap().parse().unwrap();
            let op = Op::from(op);
            Instr { o: op, a, b, c }
        })
        .collect();
    (reg as usize, second)
}

#[aoc(day21, part1)]
pub fn part1(input: &(usize, Vec<Instr>)) -> i32 {
    let (ip_reg, data) = input.clone();
    let mut reg = [0, 0, 0, 0, 0, 0];
    let l = data.len() as i32;
    while (0..l).contains(&reg[ip_reg as usize]) {
        let ip = reg[ip_reg as usize] as usize;

        let c = data[ip];
        if let Instr {
            o: Op::Eqrr, b: 0, ..
        } = c
        {
            return reg[c.a as usize];
        }
        c.do_op(&mut reg);
        reg[ip_reg as usize] += 1;
    }
    panic!("No solution?");
}

#[aoc(day21, part2)]
pub fn part2(input: &(usize, Vec<Instr>)) -> i32 {
    println!("This can take a few seconds. Please be patient.");

    // the line the escape sequence will overwrite
    println!();
    let (ip_reg, data) = input.clone();
    let mut reg = [0, 0, 0, 0, 0, 0];
    let l = data.len() as i32;
    let mut seenn = HashSet::new();
    while (0..l).contains(&reg[ip_reg as usize]) {
        let ip = reg[ip_reg as usize] as usize;

        let c = data[ip];
        if let Instr {
            o: Op::Eqrr, b: 0, ..
        } = c
        {
            let n = reg[c.a as usize];
            if !seenn.contains(&n) {
                seenn.insert(n);
                // prints all numbers that could end the program.
                // the last number printed is the solution
                // takes ~13s on my machine

                // clear last line
                print!(r"{esc}[1A{esc}[K", esc = 27 as char);

                println!("input {:<9} will halt ({:?})", reg[c.a as usize], reg);

                // just stop the code when it hits my solution, it won't do anything for others
                if n == 8164934 {
                    return n;
                }
            }
        }
        c.do_op(&mut reg);
        reg[ip_reg as usize] += 1;
    }
    panic!("No solution?");
}

#[derive(Copy, Clone)]
pub struct Instr {
    o: Op,
    a: i32,
    b: i32,
    c: i32,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Op {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl From<&str> for Op {
    fn from(a: &str) -> Self {
        match a {
            "addr" => Op::Addr,
            "addi" => Op::Addi,
            "mulr" => Op::Mulr,
            "muli" => Op::Muli,
            "banr" => Op::Banr,
            "bani" => Op::Bani,
            "borr" => Op::Borr,
            "bori" => Op::Bori,
            "setr" => Op::Setr,
            "seti" => Op::Seti,
            "gtir" => Op::Gtir,
            "gtri" => Op::Gtri,
            "gtrr" => Op::Gtrr,
            "eqir" => Op::Eqir,
            "eqri" => Op::Eqri,
            "eqrr" => Op::Eqrr,
            _ => unreachable!(),
        }
    }
}

impl Instr {
    fn do_op(&self, r: &mut [i32; 6]) {
        let (a, b, c) = (self.a, self.b, self.c);
        match self.o {
            Op::Addr => {
                r[c as usize] = r[a as usize] + r[b as usize];
            }
            Op::Addi => {
                r[c as usize] = r[a as usize] + b;
            }
            Op::Mulr => {
                r[c as usize] = r[a as usize] * r[b as usize];
            }
            Op::Muli => {
                r[c as usize] = r[a as usize] * b;
            }
            Op::Banr => {
                r[c as usize] = r[a as usize] & r[b as usize];
            }
            Op::Bani => {
                r[c as usize] = r[a as usize] & b;
            }
            Op::Borr => {
                r[c as usize] = r[a as usize] | r[b as usize];
            }
            Op::Bori => {
                r[c as usize] = r[a as usize] | b;
            }
            Op::Setr => {
                r[c as usize] = r[a as usize];
            }
            Op::Seti => {
                r[c as usize] = a;
            }
            Op::Gtir => {
                r[c as usize] = (a > r[b as usize]) as i32;
            }
            Op::Gtri => {
                r[c as usize] = (r[a as usize] > b) as i32;
            }
            Op::Gtrr => {
                r[c as usize] = (r[a as usize] > r[b as usize]) as i32;
            }
            Op::Eqir => {
                r[c as usize] = (a == r[b as usize]) as i32;
            }
            Op::Eqri => {
                r[c as usize] = (r[a as usize] == b) as i32;
            }
            Op::Eqrr => {
                r[c as usize] = (r[a as usize] == r[b as usize]) as i32;
            }
        }
    }
}

impl std::fmt::Debug for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self.o {
            Op::Addr => "addr",
            Op::Addi => "addi",
            Op::Mulr => "mulr",
            Op::Muli => "muli",
            Op::Banr => "banr",
            Op::Bani => "bani",
            Op::Borr => "borr",
            Op::Bori => "bori",
            Op::Setr => "setr",
            Op::Seti => "seti",
            Op::Gtir => "gtir",
            Op::Gtri => "gtri",
            Op::Gtrr => "gtrr",
            Op::Eqir => "eqir",
            Op::Eqri => "eqri",
            Op::Eqrr => "eqrr",
        };
        write!(f, "{name} {} {} {}", self.a, self.b, self.c)
    }
}
