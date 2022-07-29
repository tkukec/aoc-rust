use std::fmt::Write as _;

fn solve(input: &str, n: u8) -> usize {
    let mut thing = input.to_owned();
    for _ in 0..n {
        let mut new = String::new();
        let mut last_char = thing.chars().next().unwrap();
        let mut cnt = 0;
        for chr in thing.chars() {
            if chr == last_char {
                cnt += 1;
            } else {
                let _ = write!(new, "{}{}", cnt, last_char);
                cnt = 1;
                last_char = chr;
            }
        }
        let _ = write!(new, "{}{}", cnt, last_char);
        thing = new;
    }
    thing.len()
}
#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    solve(input, 40)
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    solve(input, 50)
}
