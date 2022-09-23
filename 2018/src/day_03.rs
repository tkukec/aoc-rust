use regex::Regex;
use std::{
    cmp::{max, min},
    collections::HashSet,
};
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Piece {
    id: i32,
    // start x, end x inclusive
    x: (i32, i32),
    // start y end y inclusive
    y: (i32, i32),
}

#[aoc_generator(day03)]
pub fn generate(input: &str) -> Vec<Piece> {
    let line_reg = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    input
        .lines()
        .map(|l| {
            let res = line_reg.captures(l).unwrap();
            let x_s = res[2].parse::<i32>().unwrap();
            let y_s = res[3].parse::<i32>().unwrap();
            Piece {
                id: res[1].parse().unwrap(),
                x: (x_s, x_s + res[4].parse::<i32>().unwrap() - 1),
                y: (y_s, y_s + res[5].parse::<i32>().unwrap() - 1),
            }
        })
        .collect()
}

fn overlap(a: (i32, i32), b: (i32, i32)) -> Option<(i32, i32)> {
    if min(a.1, b.1) >= max(a.0, b.0) {
        Some((max(a.0, b.0), min(a.1, b.1)))
    } else {
        None
    }
}

#[aoc(day03, part1)]
pub fn part1(input: &[Piece]) -> usize {
    // a set of overlapping points
    let mut overlapped: HashSet<(i32, i32)> = HashSet::new();
    for a in input.iter() {
        for b in input {
            if a == b {
                continue;
            }
            if let Some(xo) = overlap(a.x, b.x) {
                if let Some(yo) = overlap(a.y, b.y) {
                    (xo.0..=xo.1).for_each(|x| {
                        (yo.0..=yo.1).for_each(|y| {
                            overlapped.insert((x, y));
                        })
                    })
                }
            }
        }
    }
    overlapped.len()
}

fn contains(x: Piece, y: (i32, i32)) -> bool {
    x.x.0 <= y.0 && y.0 <= x.x.1 && x.y.0 <= y.1 && y.1 <= x.y.1
}

#[aoc(day03, part2)]
pub fn part2(input: &[Piece]) -> i32 {
    // a set of overlapping points
    let mut overlapped: HashSet<(i32, i32)> = HashSet::new();
    for a in input.iter() {
        for b in input {
            if a == b {
                continue;
            }
            if let Some(xo) = overlap(a.x, b.x) {
                if let Some(yo) = overlap(a.y, b.y) {
                    (xo.0..=xo.1).for_each(|x| {
                        (yo.0..=yo.1).for_each(|y| {
                            overlapped.insert((x, y));
                        })
                    })
                }
            }
        }
    }
    input
        .iter()
        .find(|x| !overlapped.iter().any(|p| contains(**x, *p)))
        .unwrap()
        .id
}
