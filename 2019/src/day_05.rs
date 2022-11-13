use std::collections::VecDeque;

use crate::intcode::exec_at;
#[aoc(day05, part1)]
pub fn part1(input: &str) -> i64 {
    let mut a: Vec<i64> = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut ptr = 0;

    let mut last_out = 0;
    let mut inp = VecDeque::from([1]);
    // last param is only for day 9
    while let (Some(new_ptr), output) = exec_at(&mut a, ptr, &mut inp, &mut 0) {
        if let Some(output) = output {
            last_out = output;
        }
        ptr = new_ptr;
    }

    last_out
}

#[aoc(day05, part2)]
pub fn part2(input: &str) -> i64 {
    let mut a: Vec<i64> = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut ptr = 0;
    let mut last_out = 0;
    let mut inp = VecDeque::from([5]);
    while let (Some(new_ptr), output) = exec_at(&mut a, ptr, &mut inp, &mut 0) {
        if let Some(output) = output {
            last_out = output;
        }
        ptr = new_ptr;
    }

    last_out
}
