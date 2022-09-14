use std::collections::{HashMap, VecDeque};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum Data {
    Reg(char),
    Num(i64),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Instr {
    Snd(Data),
    Set(Data, Data),
    Add(Data, Data),
    Mul(Data, Data),
    Mod(Data, Data),
    Rcv(Data),
    Jgz(Data, Data),
}

fn to_data(input: &str) -> Data {
    if let Ok(i) = input.parse() {
        Data::Num(i)
    } else {
        Data::Reg(input.chars().next().unwrap())
    }
}

fn get_data(input: Data, registers: &HashMap<Data, i64>) -> i64 {
    match input {
        Data::Num(i) => i,
        x => registers[&x],
    }
}

fn i_to_d(input: &mut dyn Iterator<Item = &str>) -> Data {
    to_data(input.next().unwrap())
}
#[aoc_generator(day18)]
pub fn generate(input: &str) -> Vec<Instr> {
    let mut out = vec![];
    for mut l in input.lines().map(|x| x.split(' ')) {
        out.push(match l.next().unwrap() {
            "snd" => Instr::Snd(i_to_d(&mut l)),
            "set" => Instr::Set(i_to_d(&mut l), i_to_d(&mut l)),
            "add" => Instr::Add(i_to_d(&mut l), i_to_d(&mut l)),
            "mul" => Instr::Mul(i_to_d(&mut l), i_to_d(&mut l)),
            "mod" => Instr::Mod(i_to_d(&mut l), i_to_d(&mut l)),
            "jgz" => Instr::Jgz(i_to_d(&mut l), i_to_d(&mut l)),
            "rcv" => Instr::Rcv(i_to_d(&mut l)),
            _ => unreachable!(),
        })
    }
    out
}

#[aoc(day18, part1)]
pub fn part1(input: &[Instr]) -> i64 {
    let mut ptr = 0i64;
    let len = input.len() as i64;
    let mut last = 0i64;
    let mut r = HashMap::new();
    while (0..len).contains(&ptr) {
        match &input[ptr as usize] {
            Instr::Snd(x) => {
                last = get_data(*x, &r);
            }
            Instr::Set(x, y) => {
                r.insert(*x, get_data(*y, &r));
            }
            Instr::Rcv(x) => {
                if get_data(*x, &r) != 0 {
                    return last;
                }
            }
            Instr::Add(x, y) => {
                *r.entry(*x).or_insert(0) += get_data(*y, &r);
            }
            Instr::Mul(x, y) => {
                *r.entry(*x).or_insert(0) *= get_data(*y, &r);
            }
            Instr::Mod(x, y) => {
                *r.entry(*x).or_insert(0) %= get_data(*y, &r);
            }
            Instr::Jgz(x, y) => {
                if get_data(*x, &r) > 0 {
                    ptr += get_data(*y, &r) - 1;
                }
            }
        }
        ptr += 1;
    }
    unreachable!()
}

#[aoc(day18, part2)]
pub fn part2(input: &[Instr]) -> i32 {
    let mut ptr0 = 0i64;
    let mut ptr1 = 0i64;

    let mut q0 = VecDeque::new();
    let mut q1 = VecDeque::new();

    let mut cnt = 0;

    let mut r0 = HashMap::new();
    r0.insert(Data::Reg('p'), 0);
    let mut r1 = HashMap::new();
    r1.insert(Data::Reg('p'), 1);
    loop {
        let mut in_deadlock = false;
        {
            match &input[ptr0 as usize] {
                Instr::Snd(x) => {
                    q1.push_back(get_data(*x, &r0));
                }
                Instr::Set(x, y) => {
                    r0.insert(*x, get_data(*y, &r0));
                }
                Instr::Rcv(x) => {
                    if let Some(v) = q0.pop_front() {
                        r0.insert(*x, v);
                    } else {
                        in_deadlock = true;
                        ptr0 -= 1;
                    }
                }
                Instr::Add(x, y) => {
                    *r0.entry(*x).or_insert(0) += get_data(*y, &r0);
                }
                Instr::Mul(x, y) => {
                    *r0.entry(*x).or_insert(0) *= get_data(*y, &r0);
                }
                Instr::Mod(x, y) => {
                    *r0.entry(*x).or_insert(0) %= get_data(*y, &r0);
                }
                Instr::Jgz(x, y) => {
                    if get_data(*x, &r0) > 0 {
                        ptr0 += get_data(*y, &r0) - 1;
                    }
                }
            }
            ptr0 += 1;
        }
        {
            match &input[ptr1 as usize] {
                Instr::Snd(x) => {
                    q0.push_back(get_data(*x, &r1));
                    cnt += 1;
                }
                Instr::Set(x, y) => {
                    r1.insert(*x, get_data(*y, &r1));
                }
                Instr::Rcv(x) => {
                    if let Some(v) = q1.pop_front() {
                        r1.insert(*x, v);
                    } else if in_deadlock {
                        return cnt;
                    } else {
                        ptr1 -= 1;
                    }
                }
                Instr::Add(x, y) => {
                    *r1.entry(*x).or_insert(0) += get_data(*y, &r1);
                }
                Instr::Mul(x, y) => {
                    *r1.entry(*x).or_insert(0) *= get_data(*y, &r1);
                }
                Instr::Mod(x, y) => {
                    *r1.entry(*x).or_insert(0) %= get_data(*y, &r1);
                }
                Instr::Jgz(x, y) => {
                    if get_data(*x, &r1) > 0 {
                        ptr1 += get_data(*y, &r1) - 1;
                    }
                }
            }
            ptr1 += 1;
        }
    }
}
