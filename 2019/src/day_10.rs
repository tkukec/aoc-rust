use std::{
    collections::HashSet,
    f64::consts::{FRAC_PI_2, PI},
};

use itertools::Itertools;

type Point = (i32, i32);

#[aoc_generator(day10)]
fn generate(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, x)| *x == '#')
                .map(move |(j, _)| (j as i32, i as i32))
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &[Point]) -> usize {
    let input = HashSet::from_iter(input.iter().copied());

    input
        .iter()
        .map(|x| count_line_of_sight(*x, &input))
        .max()
        .unwrap()
}

#[aoc(day10, part2)]
pub fn part2(input: &[Point]) -> i32 {
    let input = HashSet::from_iter(input.iter().copied());

    let best = *input
        .iter()
        .max_by_key(|x| count_line_of_sight(**x, &input))
        .unwrap();

    let in_line_of_sight = get_all_line_of_sight(best, &input);
    let last = in_line_of_sight
        .iter()
        .copied()
        // angle stuff
        .map(|x| (x, (angle(best, x) + FRAC_PI_2).rem_euclid(2.0 * PI)))
        .sorted_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
        .nth(199)
        .unwrap();
    last.0 .0 * 100 + last.0 .1
}

fn angle(a: Point, b: Point) -> f64 {
    let diff = (b.0 - a.0, b.1 - a.1);
    (diff.1 as f64).atan2(diff.0 as f64)
}

fn has_line_of_sight(a: Point, b: Point, grid: &HashSet<Point>) -> bool {
    let diff = (b.0 - a.0, b.1 - a.1);
    let gcd = gcd(diff.0.abs(), diff.1.abs());
    let diff = (diff.0 / gcd, diff.1 / gcd);
    let mut check = (a.0 + diff.0, a.1 + diff.1);
    while check != b {
        if grid.contains(&check) {
            return false;
        }
        check = (check.0 + diff.0, check.1 + diff.1);
    }
    true
}

fn count_line_of_sight(pt: Point, grid: &HashSet<Point>) -> usize {
    grid.iter()
        .filter(|x| **x != pt)
        .filter(|x| has_line_of_sight(pt, **x, grid))
        .count()
}
fn get_all_line_of_sight(pt: Point, grid: &HashSet<Point>) -> HashSet<Point> {
    grid.iter()
        .filter(|x| **x != pt)
        .filter(|x| has_line_of_sight(pt, **x, grid))
        .copied()
        .collect()
}

// https://github.com/frewsxcv/rust-gcd/blob/ab61dfd06b59f639c7acd6dc8b0493ac868e637c/src/lib.rs#L36
// dealing with only positive i32's means less casts in the code
fn gcd(mut u: i32, mut v: i32) -> i32 {
    assert!(u >= 0);
    assert!(v >= 0);
    if u == v {
        return u;
    }
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }

    let shift = (u | v).trailing_zeros();
    u >>= shift;
    v >>= shift;
    u >>= u.trailing_zeros();

    loop {
        v >>= v.trailing_zeros();

        if u > v {
            std::mem::swap(&mut u, &mut v);
        }

        v -= u; // here v >= u

        if v == 0 {
            break;
        }
    }

    u << shift
}
