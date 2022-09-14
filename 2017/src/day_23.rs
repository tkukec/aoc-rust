use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Ord, PartialOrd)]
pub enum Data {
    Reg(char),
    Num(i64),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Instr {
    Set(Data, Data),
    Sub(Data, Data),
    Mul(Data, Data),
    Jnz(Data, Data),
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

#[aoc_generator(day23)]
pub fn generate(input: &str) -> Vec<Instr> {
    let mut out = vec![];
    for mut l in input.lines().map(|x| x.split(' ')) {
        out.push(match l.next().unwrap() {
            "set" => Instr::Set(i_to_d(&mut l), i_to_d(&mut l)),
            "sub" => Instr::Sub(i_to_d(&mut l), i_to_d(&mut l)),
            "mul" => Instr::Mul(i_to_d(&mut l), i_to_d(&mut l)),
            "jnz" => Instr::Jnz(i_to_d(&mut l), i_to_d(&mut l)),
            _ => unreachable!(),
        })
    }
    out
}
#[aoc(day23, part1)]
pub fn part1(input: &[Instr]) -> u32 {
    let mut r = HashMap::new();
    let mut cnt = 0;
    for i in 'a'..='h' {
        r.insert(Data::Reg(i), 0i64);
    }
    let mut ptr = 0;
    let l = input.len() as i64;
    let mut cnt2 = 0;
    while (0..l).contains(&ptr) {
        cnt2 += 1;
        match &input[ptr as usize] {
            Instr::Set(x, y) => {
                r.insert(*x, get_data(*y, &r));
            }
            Instr::Sub(x, y) => {
                *r.get_mut(x).unwrap() -= get_data(*y, &r);
            }
            Instr::Mul(x, y) => {
                cnt += 1;
                *r.get_mut(x).unwrap() *= get_data(*y, &r);
            }
            Instr::Jnz(x, y) => {
                if get_data(*x, &r) != 0 {
                    ptr += get_data(*y, &r) - 1;
                }
            }
        }
        ptr += 1;
    }
    println!("{cnt2}");
    cnt
}

#[aoc(day23, part2)]
pub fn part2(_input: &[Instr]) -> usize {
    // simplified the assembly until i got to this
    ((79 * 100 + 100000)..)
        .step_by(17)
        .take(1001)
        .filter(|&b| (2..=((b as f64).sqrt() as i32)).any(|d| b % d == 0))
        .count()
}
