use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    DealNewStack,
    DealWithIncr(i32),
    Cut(i32),
}
impl Op {
    fn do_op(&self, stack: &mut VecDeque<u16>) {
        match *self {
            Op::DealNewStack => {
                stack.make_contiguous().reverse();
            }
            Op::DealWithIncr(i) => {
                let mut new = VecDeque::from_iter(std::iter::repeat(u16::MAX).take(stack.len()));
                let mut ptr = 0;
                for x in stack.iter() {
                    new[ptr % stack.len()] = *x;
                    ptr += i as usize;
                }
                std::mem::swap(&mut new, stack);
            }
            Op::Cut(i) => {
                if i > 0 {
                    stack.rotate_left(i as usize);
                } else {
                    stack.rotate_right(i.unsigned_abs() as usize);
                }
            }
        }
    }
}

#[aoc_generator(day22)]
fn generate(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|x| {
            if x == "deal into new stack" {
                Op::DealNewStack
            } else if x.starts_with('c') {
                Op::Cut(x.split(' ').last().unwrap().parse().unwrap())
            } else {
                Op::DealWithIncr(x.split(' ').last().unwrap().parse().unwrap())
            }
        })
        .collect()
}
#[aoc(day22, part1)]
pub fn part1(input: &[Op]) -> usize {
    let mut stack = (0..10007).collect();
    for x in input {
        x.do_op(&mut stack);
    }
    stack.iter().position(|x| *x == 2019).unwrap()
}

#[aoc(day22, part2)]
pub fn part2(_input: &[Op]) -> u32 {
    // no
    panic!("Not happening, sorry")
}
