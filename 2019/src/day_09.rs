use crate::intcode::exec_at;
use std::collections::VecDeque;

#[aoc(day09, part1)]
pub fn part1(input: &str) -> i64 {
    let mut a: Vec<i64> = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    a.append(&mut vec![0; 500]);
    let mut ptr = 0;
    let mut rel_off = 0;

    let mut last_out = 0;
    let mut inp = VecDeque::from([1]);
    while let (Some(new_ptr), output) = exec_at(&mut a, ptr, &mut inp, &mut rel_off) {
        if let Some(output) = output {
            last_out = output;
        }
        ptr = new_ptr;
    }

    last_out
}

#[aoc(day09, part2)]
pub fn part2(input: &str) -> i64 {
    let mut a: Vec<i64> = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    a.append(&mut vec![0; 500]);
    let mut ptr = 0;
    let mut rel_off = 0;

    let mut last_out = 0;
    let mut inp = VecDeque::from([2]);
    while let (Some(new_ptr), output) = exec_at(&mut a, ptr, &mut inp, &mut rel_off) {
        if let Some(output) = output {
            last_out = output;
        }
        ptr = new_ptr;
    }

    last_out
}
