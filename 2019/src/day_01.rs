#[aoc(day01, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|x| x.parse::<u32>().unwrap() / 3 - 2)
        .sum()
}

#[aoc(day01, part2)]
pub fn part2(input: &str) -> u32 {
    fn calc_fuel(x: u32) -> u32 {
        // saturating sub if x / 3 < 2
        match (x / 3).saturating_sub(2) {
            0 => 0,
            a => a + calc_fuel(a),
        }
    }
    input
        .lines()
        .map(|x| calc_fuel(x.parse::<u32>().unwrap()))
        .sum()
}
