use itertools::Itertools;
use regex::Regex;

use std::{collections::HashMap, convert::AsMut};

// really useful function from SO
// https://stackoverflow.com/a/37679019
fn clone_into_array<A, T>(slice: &[T]) -> A
where
    A: Sized + Default + AsMut<[T]>,
    T: Clone,
{
    let mut a = Default::default();
    // type inference magic, i don't know if this is
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}

fn get_matches(a: Measurement) -> Vec<Op> {
    OP_VARIANTS
        .iter()
        .copied()
        .filter(|o| o.do_op(a.instr.a, a.instr.b, a.instr.c, a.before) == a.after)
        .collect()
}

#[aoc_generator(day16)]
pub fn generate(input: &str) -> (Vec<Measurement>, Vec<Instr>) {
    let (first, second) = input.split_once("\n\n\n\n").expect("Bad input");
    let first: Vec<Measurement> = first.split("\n\n").map(Measurement::from).collect();
    let second: Vec<Instr> = second
        .lines()
        .map(|x| {
            Instr::from(clone_into_array::<[i32; 4], i32>(
                x.split(' ')
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<_>>()
                    .as_slice(),
            ))
        })
        .collect();
    (first, second)
}

#[aoc(day16, part1)]
pub fn part1(input: &(Vec<Measurement>, Vec<Instr>)) -> usize {
    let first = input.0.to_vec();

    first.iter().filter(|x| get_matches(**x).len() >= 3).count()
}

fn find_opcodes(m: Vec<Measurement>, known: &mut HashMap<i32, Op>) -> HashMap<i32, Op> {
    let found: Vec<(i32, Op)> = m
        .iter()
        .map(|x| {
            let mut a = get_matches(*x);
            a.retain(|x| !known.values().contains(x));
            (x.instr.o, a)
        })
        .filter(|x| x.1.len() == 1)
        .map(|x| (x.0, x.1[0]))
        .filter(|x| !known.values().contains(&x.1))
        .unique()
        .collect();
    known.extend(found.iter().copied());
    let mut next = m;
    next.retain(|c| !found.iter().any(|x| x.0 == c.instr.o));
    if !next.is_empty() {
        if found.is_empty() {
            // Shouldn't happen with AoC inputs, I hope
            panic!("Code got stuck. You might need an actual csp solver.");
        }
        println!("Found {}, remaining {}", found.len(), next.len());
        find_opcodes(next, known)
    } else {
        known.clone()
    }
}

#[aoc(day16, part2)]
pub fn part2(input: &(Vec<Measurement>, Vec<Instr>)) -> i32 {
    let (m, p) = input.clone();

    let mut known = HashMap::new();
    println!("{}", m.len());
    let data = find_opcodes(m, &mut known);

    let mut reg = [0; 4];
    for l in p {
        reg = data[&l.o].do_op(l.a, l.b, l.c, reg);
    }
    reg[0]
}

#[derive(Copy, Clone, Debug)]
pub struct Instr {
    o: i32,
    a: i32,
    b: i32,
    c: i32,
}

impl From<[i32; 4]> for Instr {
    fn from(i: [i32; 4]) -> Self {
        Instr {
            o: i[0],
            a: i[1],
            b: i[2],
            c: i[3],
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Measurement {
    before: [i32; 4],
    after: [i32; 4],
    instr: Instr,
}

impl From<&str> for Measurement {
    fn from(i: &str) -> Self {
        let [a, b, c] = match i.split('\n').collect::<Vec<&str>>().as_slice() {
            [a, b, c] => [*a, *b, *c],
            _ => panic!("Bad string"),
        };
        let r1 = Regex::new(r"(\d+),? (\d+),? (\d+),? (\d+)").unwrap();
        let a = r1.captures(a).expect("Bad string");
        let a: Vec<i32> = (1..=4)
            // these unwraps should never fail, because the Regex has 4 groups of digits
            .map(|x| a.get(x).unwrap().as_str().parse::<i32>().unwrap())
            .collect();

        let b = r1.captures(b).expect("Bad string");
        let b: Vec<i32> = (1..=4)
            .map(|x| b.get(x).unwrap().as_str().parse::<i32>().unwrap())
            .collect();

        let c = r1.captures(c).expect("Bad string");
        let c: Vec<i32> = (1..=4)
            .map(|x| c.get(x).unwrap().as_str().parse::<i32>().unwrap())
            .collect();

        Measurement {
            before: clone_into_array(&a),
            after: clone_into_array(&c),
            instr: Instr::from(clone_into_array::<[i32; 4], i32>(&b)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

const OP_VARIANTS: [Op; 16] = [
    Op::Addr,
    Op::Addi,
    Op::Mulr,
    Op::Muli,
    Op::Banr,
    Op::Bani,
    Op::Borr,
    Op::Bori,
    Op::Setr,
    Op::Seti,
    Op::Gtir,
    Op::Gtri,
    Op::Gtrr,
    Op::Eqir,
    Op::Eqri,
    Op::Eqrr,
];

impl Op {
    fn do_op(&self, a: i32, b: i32, c: i32, mut r: [i32; 4]) -> [i32; 4] {
        match self {
            Op::Addr => {
                r[c as usize] = r[a as usize] + r[b as usize];
                r
            }
            Op::Addi => {
                r[c as usize] = r[a as usize] + b;
                r
            }
            Op::Mulr => {
                r[c as usize] = r[a as usize] * r[b as usize];
                r
            }
            Op::Muli => {
                r[c as usize] = r[a as usize] * b;
                r
            }
            Op::Banr => {
                r[c as usize] = r[a as usize] & r[b as usize];
                r
            }
            Op::Bani => {
                r[c as usize] = r[a as usize] & b;
                r
            }
            Op::Borr => {
                r[c as usize] = r[a as usize] | r[b as usize];
                r
            }
            Op::Bori => {
                r[c as usize] = r[a as usize] | b;
                r
            }
            Op::Setr => {
                r[c as usize] = r[a as usize];
                r
            }
            Op::Seti => {
                r[c as usize] = a;
                r
            }
            Op::Gtir => {
                r[c as usize] = (a > r[b as usize]) as i32;
                r
            }
            Op::Gtri => {
                r[c as usize] = (r[a as usize] > b) as i32;
                r
            }
            Op::Gtrr => {
                r[c as usize] = (r[a as usize] > r[b as usize]) as i32;
                r
            }
            Op::Eqir => {
                r[c as usize] = (a == r[b as usize]) as i32;
                r
            }
            Op::Eqri => {
                r[c as usize] = (r[a as usize] == b) as i32;
                r
            }
            Op::Eqrr => {
                r[c as usize] = (r[a as usize] == r[b as usize]) as i32;
                r
            }
        }
    }
}
