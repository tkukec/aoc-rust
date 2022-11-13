use crate::intcode::exec_at;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

type Point = (i32, i32);
#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let mut code: Vec<i64> = input.split(',').map(|x| x.parse().unwrap()).collect();

    code.append(&mut vec![0; 500]);
    let mut ptr = 0;
    let mut rel_off = 0;
    let mut grid = [[false; 150]; 150];
    let mut input = VecDeque::from([0]);
    let mut cur_pos = (0, 0);
    let mut cur_dir = Dir::Up;
    let mut out1 = None;
    let mut painted = HashSet::new();
    while let (Some(new_ptr), output) = exec_at(&mut code, ptr, &mut input, &mut rel_off) {
        ptr = new_ptr;
        if output.is_some() {
            if out1.is_none() {
                out1 = output;
            } else {
                let out2 = output;
                let o1 = out1.unwrap();
                let o2 = out2.unwrap();

                grid[(cur_pos.1 + 75) as usize][(cur_pos.0 + 75) as usize] = o1 != 0;
                painted.insert(cur_pos);

                match o2 {
                    0 => cur_dir.turn_left(),
                    1 => cur_dir.turn_right(),
                    _ => panic!("Invalid output recieved"),
                }
                step_in_dir(&mut cur_pos, cur_dir);

                input.push_back(grid[(cur_pos.1 + 75) as usize][(cur_pos.0 + 75) as usize] as i64);

                out1 = None;
            }
        }
    }
    painted.len()
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> String {
    let mut code: Vec<i64> = input.split(',').map(|x| x.parse().unwrap()).collect();

    code.append(&mut vec![0; 1000]);
    let mut ptr = 0;
    let mut rel_off = 0;
    let mut grid = [[false; 200]; 200];
    grid[0][0] = true;
    let mut input = VecDeque::from([1]);
    let mut cur_pos = (0, 0);
    let mut cur_dir = Dir::Up;
    let mut out1 = None;
    while let (Some(new_ptr), output) = exec_at(&mut code, ptr, &mut input, &mut rel_off) {
        ptr = new_ptr;
        if output.is_some() {
            if out1.is_none() {
                out1 = output;
            } else {
                let out2 = output;
                let o1 = out1.unwrap();
                let o2 = out2.unwrap();

                grid[(cur_pos.1 + 100) as usize][(cur_pos.0 + 100) as usize] = o1 != 0;

                match o2 {
                    0 => cur_dir.turn_left(),
                    1 => cur_dir.turn_right(),
                    _ => panic!("Invalid output recieved"),
                }
                step_in_dir(&mut cur_pos, cur_dir);

                input
                    .push_back(grid[(cur_pos.1 + 100) as usize][(cur_pos.0 + 100) as usize] as i64);

                out1 = None;
            }
        }
    }
    grid[99..=105]
        .iter()
        .map(|x| {
            x[100..150]
                .iter()
                .map(|p| if *p { '#' } else { ' ' })
                .collect::<String>()
        })
        .join("\n")
}

fn step_in_dir(pt: &mut Point, dir: Dir) {
    match dir {
        Dir::Up => pt.1 -= 1,
        Dir::Right => pt.0 += 1,
        Dir::Down => pt.1 += 1,
        Dir::Left => pt.0 -= 1,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn turn_left(&mut self) {
        *self = match self {
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
        }
    }
    fn turn_right(&mut self) {
        *self = match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}
