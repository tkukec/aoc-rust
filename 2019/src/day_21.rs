use std::collections::VecDeque;

use crate::intcode::exec_at;
#[aoc(day21, part1)]
pub fn part1(input: &str) -> i64 {
    let mut code: Vec<i64> = input.split(',').map(|x| x.parse().unwrap()).collect();
    code.append(&mut vec![0; 100]);
    let mut ptr = 0;
    let mut rel_off = 0;
    let i = "NOT A J
NOT B T
OR J T
NOT C J
OR J T
AND D T
AND T J
OR T J
WALK
";
    let mut input = VecDeque::from_iter(i.bytes().map(|x| x as i64));
    while let (Some(new_ptr), out) = exec_at(&mut code, ptr, &mut input, &mut rel_off) {
        ptr = new_ptr;
        if let Some(c) = out {
            match u8::try_from(c) {
                Ok(c) => print!("{}", c as char),
                Err(_) => return c,
            }
        }
    }
    println!();
    panic!("failed");
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> i64 {
    let mut code: Vec<i64> = input.split(',').map(|x| x.parse().unwrap()).collect();
    code.append(&mut vec![0; 100]);
    let mut ptr = 0;
    let mut rel_off = 0;
    let i = "NOT A J
NOT B T
OR J T
NOT C J
OR J T
AND D T
AND T J
OR T J
AND H J
NOT A T 
OR T J
RUN
";
    let mut input = VecDeque::from_iter(i.bytes().map(|x| x as i64));
    while let (Some(new_ptr), out) = exec_at(&mut code, ptr, &mut input, &mut rel_off) {
        ptr = new_ptr;
        if let Some(c) = out {
            match u8::try_from(c) {
                Ok(c) => print!("{}", c as char),
                Err(_) => return c,
            }
        }
    }
    println!();
    panic!("failed");
}

// when to jump
//
// _??# -> jump
// ??_# -> jump
// ?_?# -> jump
//
//
// NOT A J
// NOT B T
// OR J T
// NOT C J
// OR J T
// AND D T
// AND T J
// OR T J
//
//
// part 2 when to jump
//
//
//
//
// NOT A J
// NOT B T
// OR J T
// NOT C J
// OR J T
// AND D T
// AND T J
// OR T J
// AND H J // also check that H is not a hole so we can jump from D to H
//
// NOT A T // just jump if you're gonna fall in the hole next turn
// OR T J
//
//
