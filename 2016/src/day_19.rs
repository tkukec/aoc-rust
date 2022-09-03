#[aoc(day19, part1)]
pub fn part1(input: &str) -> u32 {
    let i: u32 = input.parse().unwrap();
    let mut i2 = i;
    let mut cnt = 0;
    while i2 > 1 {
        cnt += 1;
        i2 >>= 1;
    }
    ((i - 2u32.pow(cnt)) << 1) + 1
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> u32 {
    fn highest_pow_of_3(x: u32) -> u32 {
        let mut c = 0;
        while 3u32.pow(c) <= x {
            c += 1;
        }
        3u32.pow(c - 1)
    }
    let i: u32 = input.parse().unwrap();
    let x = highest_pow_of_3(i);
    if x == i {
        i
    } else if i < 2 * x {
        i % x
    } else {
        x + 2 * (i % x)
    }
}

// part 1
// for small numbers n:
// n  -> f(n)
//  1 -> 1
//  2 -> 1
//  3 -> 3
//  4 -> 1
//  5 -> 3
//  6 -> 5
//  7 -> 7
//  8 -> 1
//  9 -> 3
// 10 -> 5
// 11 -> 7
// 12 -> 9
//
// this is https://oeis.org/A006257, aka the josephus problem
// the result is the original number, left shifted by one
// (most significant bit is removed and appended to the end)
//
//
//
// part 2
// n  -> f(n)
//  1 -> 1
//  2 -> 1
//  3 -> 3
//  4 -> 1
//  5 -> 2
//  6 -> 3
//  7 -> 5
//
// the oeis has 5 results for this sequence, and A334473 is what we need
// https://oeis.org/A334473 (the n-cowboy shootout problem)
// a python solution is given in the oeis, i just translated it to rust
