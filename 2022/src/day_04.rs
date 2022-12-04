use text_io::scan;
#[aoc(day04, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            // originally solved with a regex, but this is so much nicer holy shit
            let (a, b, c, d): (u32, u32, u32, u32);
            scan!(l.bytes() => "{}-{},{}-{}", a, b, c, d);
            (a, b, c, d)
        })
        .filter(|(a, b, c, d)| a <= c && b >= d || c <= a && d >= b)
        .count()
}

#[aoc(day04, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (a, b, c, d): (u32, u32, u32, u32);
            scan!(l.bytes() => "{}-{},{}-{}", a, b, c, d);
            (a, b, c, d)
        })
        .filter(|(a, b, c, d)| a <= c && c <= b || c <= a && a <= d)
        .count()
}
