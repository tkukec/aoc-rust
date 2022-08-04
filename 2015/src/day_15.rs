// fml i butchered this one
use std::cmp::{max, min};

fn generate(input: &str) -> Vec<Vec<i32>> {
    let mut out = vec![];
    for i in input.lines() {
        out.push(
            i.replace(',', "")
                .split(' ')
                .filter_map(|s| s.parse().ok())
                .collect(),
        );
    }
    out
}

fn score(input: &[Vec<i32>], f: i32, c: i32, b: i32, s: i32) -> i32 {
    let cap = input[0][0] * f + input[1][0] * c + input[2][0] * b + input[3][0] * s;
    let dur = input[0][1] * f + input[1][1] * c + input[2][1] * b + input[3][1] * s;
    let fla = input[0][2] * f + input[1][2] * c + input[2][2] * b + input[3][2] * s;
    let tex = input[0][3] * f + input[1][3] * c + input[2][3] * b + input[3][3] * s;
    cap * dur * fla * tex
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> i32 {
    // Frosting     - f 0..100
    // Candy        - c ((f*2)/5)..=(100-f)
    // Butterscotch - b (c/5)..min(100-f-c, f * 4)
    // Sugar        - s 100-f-c-b
    //
    // this narrows it down to <68k iterations, but it works only on my input...
    // making it work on all inputs would take hours of math and a billion if statements
    //
    // coding this took only 5 minutes, the math took half an hour
    let input = generate(input);
    let mut best = 0;
    for f in 0..100 {
        for c in ((f * 2) / 5)..=(100 - f) {
            for b in ((c + 2) / 5)..min(100 - f - c, f * 4) {
                let s = 100 - f - c - b;
                if s > 0 && f * 4 > b && c * 5 > f * 2 && b * 5 > c + s * 2 {
                    best = max(best, score(&input, f, c, b, s));
                }
            }
        }
    }
    best
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> i32 {
    // same as pt 1, just one more check so even less score calculations
    let input = generate(input);
    let mut best = 0;
    for f in 0..100 {
        for c in ((f * 2) / 5)..=(100 - f) {
            for b in (c / 5)..min(100 - f - c, f * 4) {
                let s = 100 - f - c - b;
                let cal = input[0][4] * f + input[1][4] * c + input[2][4] * b + input[3][4] * s;
                if s > 0 && f * 4 > b && c * 5 > f * 2 && b * 5 > c + s * 2 && cal == 500 {
                    best = max(best, score(&input, f, c, b, s));
                }
            }
        }
    }
    best
}
// METHOD
//
// Frosting:     capacity 4,  durability -2, flavor 0,  texture 0, calories 5
// Candy:        capacity 0,  durability 5,  flavor -1, texture 0, calories 8
// Butterscotch: capacity -1, durability 0,  flavor 5,  texture 0, calories 6
// Sugar:        capacity 0,  durability 0,  flavor -2, texture 2, calories 1
//
// ----- PART 1 -----
// BOUNDS (for my input):
// cap > 0
// -> f * 4 + b * (-1) > 0
// -> f * 4 > b
//
// dur > 0
// -> f * (-2) + c * 5 > 0
// -> c * 5 > f * 2
//
// fla > 0
// -> c * (-1) + b * 5 + s * (-2) > 0
// -> b * 5 > c + s * 2
//
// tex > 0
// -> s > 0
//
// f + c + b + s = 100
//
// ---
// f * 4 > b
// f * 2 < c * 5
//
// f * 4 > b
// f * 4 < c * 10
// b < f * 4 < c * 10
// ---
// b < c * 10
// b * 5 > c + s * 2 (to simplify the equation, s = 1)
//
// b < c * 10
// b * 5 > c + 2
//
// b * 5 < c * 50
// b * 5 > c + 2
//
// c + 2 < b * 5 < c * 50
// (c + 2) / 5 < b < c * 10
// c / 5 < b < c * 5
// b = (c/5)..min(100-c, c*10)
//
// LOOPS
// Frosting     - f 0..100
// Candy        - c ((f*2)/5)..=(100-f)
// Butterscotch - b (c/5)..min(100-f-c, f * 4)
// Sugar        - s 100-f-c-b
//
// (the ingredients are in the order that allows me to narrow down values the most. There might be
// an even better ordering of loops, but I didn't care enough to find it.)
//
// ----- PART 2 -----
// Copy the part one code, but add a check to make sure there are 500 calories. It even runs
// faster, because it needs to calculate less possible scores
//

fn score_brute(input: &[Vec<i32>], f: i32, c: i32, b: i32, s: i32) -> i32 {
    // need to have more checks because negative scores are now possible
    let cap = max(
        0,
        input[0][0] * f + input[1][0] * c + input[2][0] * b + input[3][0] * s,
    );
    let dur = max(
        0,
        input[0][1] * f + input[1][1] * c + input[2][1] * b + input[3][1] * s,
    );
    let fla = max(
        0,
        input[0][2] * f + input[1][2] * c + input[2][2] * b + input[3][2] * s,
    );
    let tex = max(
        0,
        input[0][3] * f + input[1][3] * c + input[2][3] * b + input[3][3] * s,
    );
    cap * dur * fla * tex
}

#[aoc(day15, part1, no_bounds)]
pub fn part1_brute(input: &str) -> i32 {
    // fuck math, rust is fast enough
    let input = generate(input);
    let mut best = 0;
    for f in 0..100 {
        for c in 0..(100 - f) {
            for b in 0..(100 - f - c) {
                let s = 100 - f - c - b;
                if s >= 0 {
                    best = max(best, score_brute(&input, f, c, b, s));
                }
            }
        }
    }
    // well shit, my bounds only narrow it down to a third of the possibilities, but it still only
    // takes 1.4ms to loop through the 171k possibilities...
    // this also works on every solution, and is easier to understand
    // 65% faster execution doesn't matter if it's only faster by a millisecond
    best
}
