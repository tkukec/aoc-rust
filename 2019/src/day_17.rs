use std::collections::VecDeque;

use crate::intcode::exec_at;
#[aoc(day17, part1)]
pub fn part1(input: &str) -> usize {
    let mut code: Vec<i64> = input
        .split(',')
        .map(|x| x.parse().unwrap())
        .chain(std::iter::repeat(0).take(5000))
        .collect();
    let mut ptr = 0;
    let mut rel_off = 0;
    let mut input = VecDeque::new();
    let mut map = String::new();
    while let (Some(new_ptr), out) = exec_at(&mut code, ptr, &mut input, &mut rel_off) {
        ptr = new_ptr;
        if let Some(c) = out {
            let c = c as u8 as char;
            map.push(c);
        }
    }

    let map: Vec<Vec<char>> = map
        .lines()
        .filter(|x| !x.is_empty())
        .map(|l| l.chars().collect())
        .collect();
    let x_size = map[0].len();
    let y_size = map.len();
    let mut score = 0;
    for y in 1..(y_size - 1) {
        for x in 1..(x_size - 1) {
            if matches!(map[y][x], '#' | '^')
                && matches!(map[y][x - 1], '#' | '^')
                && matches!(map[y][x + 1], '#' | '^')
                && matches!(map[y - 1][x], '#' | '^')
                && matches!(map[y + 1][x], '#' | '^')
            {
                score += x * y;
            }
        }
    }

    score
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> i64 {
    let initial: Vec<i64> = input
        .split(',')
        .map(|x| x.parse().unwrap())
        .chain(std::iter::repeat(0).take(5000))
        .collect();
    let mut code = initial.clone();
    let mut ptr = 0;
    let mut rel_off = 0;
    let mut input = VecDeque::new();
    let mut map = String::new();
    while let (Some(new_ptr), out) = exec_at(&mut code, ptr, &mut input, &mut rel_off) {
        ptr = new_ptr;
        if let Some(c) = out {
            let c = c as u8 as char;
            map.push(c);
        }
    }

    let map: Vec<Vec<char>> = map
        .lines()
        .filter(|x| !x.is_empty())
        .map(|l| l.chars().collect())
        .collect();
    let x_size = map[0].len();
    let mut path: VecDeque<i64> = VecDeque::new();
    let start = map.iter().flatten().position(|x| *x == '^').unwrap();
    let mut cur = (start % x_size, start / x_size);
    let mut cur_dir = Dir::Up;
    let mut line_len = 1;

    loop {
        let next = cur_dir.step(cur);

        if let Some(Some(p)) = map.get(next.1).map(|x| x.get(next.0)) {
            if *p == '#' {
                line_len += 1;
                cur = next;
                continue;
            }
        }
        path.push_back(line_len);
        let left = cur_dir.turn(Turn::Left).step(cur);
        let right = cur_dir.turn(Turn::Right).step(cur);
        if let Some(Some(p)) = map.get(left.1).map(|x| x.get(left.0)) {
            if *p == '#' {
                cur_dir = cur_dir.turn(Turn::Left);
                cur = left;
                line_len = 1;
                path.push_back('L' as i64);
                continue;
            }
        }

        if let Some(Some(p)) = map.get(right.1).map(|x| x.get(right.0)) {
            if *p == '#' {
                cur_dir = cur_dir.turn(Turn::Right);
                cur = right;
                line_len = 1;
                path.push_back('R' as i64);
                continue;
            }
        }
        path.push_back(line_len + 1);
        break;
    }
    path.pop_front().unwrap();
    path.pop_back().unwrap();
    // R, 6, L, 10, R, 8, R, 8, R, 12, L, 8, L, 10, R, 6, L, 10, R, 8, R, 8, R, 12, L, 10, R, 6, L, 10, R, 12, L, 8, L, 10, R, 12, L, 10, R, 6, L, 10, R, 6, L, 10, R, 8, R, 8, R, 12, L, 8, L, 10, R, 6, L, 10, R, 8, R, 8, R, 12, L, 10, R, 6, L, 10
    // A, B, C, C, D, E, B, A, B, C, C, D, B, A, B, D, E, B, D, B, A, B, A, B, C, C, D, E, B, A, B, C, C, D, B, A, B

    // A, B, C, C,
    // D, E, B,
    // A, B, C, C,
    // D, B, A, B,
    // D, E, B,
    // D, B, A, B,
    // A, B, C, C,
    // D, E, B,
    // A, B, C, C,
    // D, B, A, B

    // ABACBCABAC

    const A: &str = "R,6,L,10,R,8,R,8\n";
    const B: &str = "R,12,L,8,L,10\n";
    const C: &str = "R,12,L,10,R,6,L,10\n";

    const MAIN: &str = "A,B,A,C,B,C,A,B,A,C\n";
    let mut input: VecDeque<i64> = vec![MAIN, A, B, C, "n\n"]
        .iter()
        .flat_map(|x| x.chars())
        .map(|c| c as i64)
        .collect();
    let mut code = initial;
    code[0] = 2;

    let mut ptr = 0;
    let mut rel_off = 0;
    let mut o = Vec::new();
    while let (Some(new_ptr), out) = exec_at(&mut code, ptr, &mut input, &mut rel_off) {
        ptr = new_ptr;
        if let Some(c) = out {
            o.push(c);
        }
    }

    o.into_iter().last().unwrap()
}

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn(self, t: Turn) -> Self {
        match t {
            Turn::Left => match self {
                Dir::Up => Dir::Left,
                Dir::Down => Dir::Right,
                Dir::Left => Dir::Down,
                Dir::Right => Dir::Up,
            },
            Turn::Right => match self {
                Dir::Up => Dir::Right,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
                Dir::Right => Dir::Down,
            },
        }
    }
    fn step(self, pt: (usize, usize)) -> (usize, usize) {
        match self {
            Dir::Up => (pt.0, pt.1 - 1),
            Dir::Down => (pt.0, pt.1 + 1),
            Dir::Left => (pt.0 - 1, pt.1),
            Dir::Right => (pt.0 + 1, pt.1),
        }
    }
}

#[derive(Clone, Copy)]
enum Turn {
    Left,
    Right,
}
