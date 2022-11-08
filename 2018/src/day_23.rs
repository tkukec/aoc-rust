use std::str::FromStr;

type Point = (i32, i32, i32);
use regex::Regex;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct Nanobot {
    x: i32,
    y: i32,
    z: i32,
    r: u32,
}

fn get_custom<T>(cap: &regex::Captures, pos: usize) -> T
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    cap.get(pos).unwrap().as_str().parse().unwrap()
}

#[aoc_generator(day23)]
fn generate(input: &str) -> Vec<Nanobot> {
    let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    let mut out = vec![];
    for l in input.lines() {
        let a = re.captures(l).unwrap();
        out.push(Nanobot {
            x: get_custom(&a, 1),
            y: get_custom(&a, 2),
            z: get_custom(&a, 3),
            r: get_custom(&a, 4),
        });
    }
    out
}

impl Nanobot {
    fn manhattan(&self, other: &Nanobot) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
    fn in_range(&self, pt: Point) -> bool {
        self.manhattan(&Nanobot {
            x: pt.0,
            y: pt.1,
            z: pt.2,
            r: 1,
        }) <= self.r
    }

    fn scale_down(&self, amount: u32) -> Nanobot {
        let mut new = *self;
        new.x /= amount as i32;
        new.y /= amount as i32;
        new.z /= amount as i32;
        new.r /= amount;
        new
    }
}
#[aoc(day23, part1)]
pub fn part1(input: &[Nanobot]) -> usize {
    let biggest = *input.iter().max_by_key(|x| x.r).unwrap();
    input
        .iter()
        .filter(|n| n.manhattan(&biggest) <= biggest.r)
        .count()
}

// Tried to do something smarter with graph theory and clique stuff. It didn't work.
// Even installed z3 for a moment, but the documentation for the rust bindings was bad.
// Simple brute force never failed me.
#[aoc(day23, part2)]
pub fn part2(input: &[Nanobot]) -> i32 {
    fn scale_down_by(a: &[Nanobot], scale: u32) -> Vec<Nanobot> {
        a.iter().map(|x| x.scale_down(scale)).collect()
    }

    let mut best_score: usize;
    let mut best = (0, 0, 0);
    for i in (0..=7).rev() {
        let scaled = scale_down_by(input, 10u32.pow(i));
        best = (best.0 * 10, best.1 * 10, best.2 * 10);
        best_score = 0;
        for x in (best.0 - 10)..=(best.0 + 10) {
            for y in (best.1 - 10)..=(best.1 + 10) {
                for z in (best.2 - 10)..=(best.2 + 10) {
                    let count = scaled.iter().filter(|n| n.in_range((x, y, z))).count();
                    if count > best_score {
                        best_score = count;
                        best = (x, y, z);
                    }
                }
            }
        }
        println!("{best_score} - {best:?}");
    }
    best.0.abs() + best.1.abs() + best.2.abs()
}
