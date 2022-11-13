use crate::intcode::exec_at;
use itertools::Itertools;
use std::collections::VecDeque;

#[aoc(day07, part1)]
pub fn part1(input: &str) -> i64 {
    let input: Vec<i64> = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut max_out = 0;
    for x in (0..=4).permutations(5) {
        // for x in [[4, 3, 2, 1, 0]] {
        max_out = std::cmp::max(
            max_out,
            (0..5).fold(0, |state, i| {
                let mut ptr = 0;
                let mut output = 0;
                let mut code = input.clone();
                let mut input = VecDeque::from([x[i], state]);
                // last param is only for day 9
                while let (Some(new_ptr), out) = exec_at(&mut code, ptr, &mut input, &mut 0) {
                    if let Some(out) = out {
                        output = out;
                    }
                    ptr = new_ptr;
                }
                output
            }),
        );
    }

    max_out
}

#[aoc(day07, part2)]
pub fn part2(input: &str) -> i64 {
    let input: Vec<i64> = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut max_out = 0;
    for x in (5..=9).permutations(5) {
        let mut cur = 'a';

        let mut a = input.clone();
        let mut b = input.clone();
        let mut c = input.clone();
        let mut d = input.clone();
        let mut e = input.clone();
        let mut curcode = &mut a;

        let mut ainp = VecDeque::from([x[0], 0]);
        let mut binp = VecDeque::from([x[1]]);
        let mut cinp = VecDeque::from([x[2]]);
        let mut dinp = VecDeque::from([x[3]]);
        let mut einp = VecDeque::from([x[4]]);
        let mut curinp = &mut ainp;

        let mut aptr = 0;
        let mut bptr = 0;
        let mut cptr = 0;
        let mut dptr = 0;
        let mut eptr = 0;

        let mut curptr = &mut aptr;
        // store the last output from E. When A halts, it's going to the thrusters
        let mut last_out = 0;

        // just change the cur.* pointers every time you get an output
        while let (Some(new_ptr), output) = exec_at(curcode, *curptr, curinp, &mut 0) {
            *curptr = new_ptr;
            if let Some(output) = output {
                match cur {
                    'a' => {
                        cur = 'b';
                        curptr = &mut bptr;
                        curcode = &mut b;
                        curinp = &mut binp;
                    }
                    'b' => {
                        cur = 'c';
                        curptr = &mut cptr;
                        curcode = &mut c;
                        curinp = &mut cinp;
                    }
                    'c' => {
                        cur = 'd';
                        curptr = &mut dptr;
                        curcode = &mut d;
                        curinp = &mut dinp;
                    }
                    'd' => {
                        cur = 'e';
                        curptr = &mut eptr;
                        curcode = &mut e;
                        curinp = &mut einp;
                    }
                    'e' => {
                        cur = 'a';
                        last_out = output;
                        curptr = &mut aptr;
                        curcode = &mut a;
                        curinp = &mut ainp;
                    }
                    _ => unreachable!(),
                }
                // add the output to the input of the next amp
                curinp.push_back(output);
            }
        }
        max_out = std::cmp::max(max_out, last_out);
    }

    max_out
}
