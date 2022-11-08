#[aoc_generator(day19)]
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

#[aoc(day19, part1)]
pub fn part1(input: &(usize, Vec<Instr>)) -> i32 {
    let (ip_reg, data) = input.clone();
    let mut reg = [0, 0, 0, 0, 0, 0];
    let l = data.len() as i32;
    while (0..l).contains(&reg[ip_reg as usize]) {
        let ip = reg[ip_reg as usize] as usize;
        let c = data[ip];
        c.do_op(&mut reg);
        reg[ip_reg as usize] += 1;
    }
    reg[0]
}

// get the big number you need to calculate the factor sum of
fn part2_get_max(input: &(usize, Vec<Instr>)) -> i32 {
    let (ip_reg, data) = input.clone();
    let mut reg = [1, 0, 0, 0, 0, 0];
    let mut cnt = 0;

    // 30 instructions should be plenty
    while cnt < 30 {
        cnt += 1;
        let ip = reg[ip_reg as usize] as usize;
        let c = data[ip];
        c.do_op(&mut reg);
        reg[ip_reg as usize] += 1;
    }
    *reg.iter().max().unwrap()
}

#[aoc(day19, part2)]
pub fn part2_worse(input: &(usize, Vec<Instr>)) -> i32 {
    let n = part2_get_max(input);

    (1..=n).filter(|i| n % i == 0).sum()
}

// actually understanding what the code does, for part 2
//
// [1, 0, 0, 0, 0, 0]
// #ip 3         ip reg = 3;
// addi 3 16 3   r3 += 16 // do the init code
// seti 1 3 2    r2 = 1
// seti 1 0 5    r5 = 1 // outer loop start here (loop r2 1 to r4 inclusive)
// mulr 2 5 1    r1 = r2 * r5 // inner loop start here (loop r5 1 to r4 inclusive)
// eqrr 1 4 1    r1 = (r1 == r4) // check if r2 * r5 == r4
// addr 1 3 3    r3 += r1
// addi 3 1 3    r3 += 1
// addr 2 0 0    r0 += r2 // add r2 to r0 (output)
// addi 5 1 5    r5 += 1
// gtrr 5 4 1    r1 = (r5 > r4) // * inner loop end check
// addr 3 1 3    r3 += r1       // *
// seti 2 2 3    r3 = 2         // *
// addi 2 1 2    r2 += 1
// gtrr 2 4 1    r1 = (r2 > r4)  // * outer loop end check
// addr 1 3 3    r3 += r1        // *
// seti 1 1 3    r3 = 1          // *
// mulr 3 3 3    r3 = r3 * r3    // * exit program if here
// addi 4 2 4    r4 += 2    vvvv only executed at the start
// mulr 4 4 4    r4 = r4 * r4
// mulr 3 4 4    r4 *= r3
// muli 4 11 4   r4 *= 11
// addi 1 4 1    r1 += 1
// mulr 1 3 1    r1 *= r3
// addi 1 2 1    r1 += 2
// addr 4 1 4    r4 += r1
// addr 3 0 3    r3 += r0
// seti 0 2 3    r3 = 0
// setr 3 6 1    r1 = 3
// mulr 1 3 1    r1 *= r3
// addr 3 1 1    r1 += r3
// mulr 3 1 1    r1 *= r3
// muli 1 14 1   r1 *= 14
// mulr 1 3 1    r1 *= r3
// addr 4 1 4    r4 += r1
// seti 0 6 0    r0 = 0
// seti 0 9 3    r3 = 0

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
