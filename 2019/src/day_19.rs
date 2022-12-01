use crate::intcode::exec_at;
use std::collections::VecDeque;

fn run_until_output(input: &mut VecDeque<i64>, mut code: Vec<i64>) -> i64 {
    let mut ptr = 0;
    let mut rel_off = 0;
    while let (Some(new_ptr), output) = exec_at(&mut code, ptr, input, &mut rel_off) {
        ptr = new_ptr;
        if let Some(output) = output {
            return output;
        }
    }

    panic!("Program halted without output");
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> i64 {
    let mut code: Vec<i64> = input.split(',').map(|x| x.parse().unwrap()).collect();
    code.append(&mut vec![0; 100]);
    let mut input = VecDeque::from_iter((0..50).flat_map(|x| (0..50).flat_map(move |i| [i, x])));
    let mut cnt = 0;
    while !input.is_empty() {
        cnt += run_until_output(&mut input, code.clone());
    }

    cnt
}

fn is_in_beam(x: i64, y: i64, code: &[i64]) -> bool {
    let mut input = VecDeque::from([x, y]);
    run_until_output(&mut input, code.to_owned()) != 0
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> i64 {
    let mut code: Vec<i64> = input.split(',').map(|x| x.parse().unwrap()).collect();
    code.append(&mut vec![0; 100]);

    let mut x = 0;
    let mut y = 0;

    // go down until it works
    while !is_in_beam(x + 99, y, &code) {
        y += 1;
        // just go left until the point 100 below is in beam
        // if the point 100 to the right is also in beam, it's good
        // otherwise go down and try again
        while !is_in_beam(x, y + 99, &code) {
            x += 1;
        }
    }
    x * 10000 + y
}
