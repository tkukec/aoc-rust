use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum Instr {
    P(char, char),
    X(usize, usize),
    S(usize),
}
#[aoc_generator(day16)]
pub fn generate(input: &str) -> Vec<Instr> {
    input
        .split(',')
        .map(|ins| match ins.chars().next().unwrap() {
            'p' => {
                let first = ins.chars().nth(1).unwrap();
                let second = ins.chars().last().unwrap();

                Instr::P(first, second)
            }
            'x' => {
                let first_pos = ins
                    .chars()
                    .skip(1)
                    .take_while(|x| x.is_ascii_digit())
                    .collect::<String>()
                    .parse()
                    .unwrap();
                let second_pos = ins
                    .chars()
                    .skip_while(|x| x != &'/')
                    .skip(1)
                    .take_while(|x| x.is_ascii_digit())
                    .collect::<String>()
                    .parse()
                    .unwrap();
                Instr::X(first_pos, second_pos)
            }
            's' => {
                let n: usize = ins.chars().skip(1).collect::<String>().parse().unwrap();
                Instr::S(n)
            }
            _ => unreachable!(),
        })
        .collect()
}
#[aoc(day16, part1)]
pub fn part1(input: &[Instr]) -> String {
    let mut pos: VecDeque<char> = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ]
    .into();
    for ins in input {
        match ins {
            Instr::P(first, second) => {
                for a in pos.iter_mut() {
                    if a == first {
                        *a = *second;
                    } else if a == second {
                        *a = *first;
                    }
                }
            }
            Instr::X(first_pos, second_pos) => {
                pos.swap(*first_pos, *second_pos);
            }
            Instr::S(n) => {
                for _ in 0..*n {
                    let a = pos.pop_back().unwrap();
                    pos.push_front(a);
                }
            }
        }
    }
    pos.into_iter().map(|x| x.to_string()).join("")
}

// The loop actually starts at the initial position "abcdef....", so I can simplify the code a lot
#[aoc(day16, part2)]
pub fn part2(input: &[Instr]) -> String {
    let mut pos: VecDeque<char> = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ]
    .into();
    let first_pos = pos.clone();
    let first_instr = input[0];
    let mut loop_items: Vec<VecDeque<char>> = vec![];
    let i_iter = std::iter::repeat(input).take(1000000000).flatten();
    for ins in i_iter.copied() {
        if ins == first_instr && pos == first_pos && !loop_items.is_empty() {
            let loop_len = loop_items.len();
            return loop_items[1000000000 % loop_len]
                .iter()
                .map(|x| x.to_string())
                .join("");
        } else {
            loop_items.push(pos.clone());
        }

        match ins {
            Instr::P(first, second) => {
                for a in pos.iter_mut() {
                    if a == &first {
                        *a = second;
                    } else if a == &second {
                        *a = first;
                    }
                }
            }
            Instr::X(first_pos, second_pos) => {
                pos.swap(first_pos, second_pos);
            }
            Instr::S(n) => {
                for _ in 0..n {
                    let a = pos.pop_back().unwrap();
                    pos.push_front(a);
                }
            }
        }
    }
    unreachable!()
}

// How i solved it initially
#[aoc(day16, part2, old)]
pub fn part2_old(input: &[Instr]) -> String {
    let mut pos: VecDeque<char> = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ]
    .into();
    let mut seen: HashMap<(VecDeque<char>, Instr), VecDeque<char>> = HashMap::new();
    let mut first_seen_twice = VecDeque::new();
    let mut loop_items = vec![];
    let i_iter = std::iter::repeat(input).take(1000000000).flatten();
    for ins in i_iter.copied() {
        if let Some(pos_after) = seen.get(&(pos.clone(), ins)) {
            if first_seen_twice.is_empty() {
                // The beginning of the loop is found
                first_seen_twice = pos.clone();
                loop_items.push(pos.clone());
                pos = pos_after.clone();
            } else if pos != first_seen_twice {
                // Collecting the loop
                loop_items.push(pos.clone());
                pos = pos_after.clone();
            } else {
                // Back at the start of the loop
                let loop_len = loop_items.len();
                return loop_items[1000000000 % loop_len]
                    .iter()
                    .map(|x| x.to_string())
                    .join("");
            }
            continue;
        }

        let old = pos.clone();
        match ins {
            Instr::P(first, second) => {
                for a in pos.iter_mut() {
                    if *a == first {
                        *a = second;
                    } else if *a == second {
                        *a = first;
                    }
                }
            }
            Instr::X(first_pos, second_pos) => {
                pos.swap(first_pos, second_pos);
            }
            Instr::S(n) => {
                for _ in 0..n {
                    let a = pos.pop_back().unwrap();
                    pos.push_front(a);
                }
            }
        }
        seen.insert((old, ins), pos.clone());
    }
    unreachable!()
}
