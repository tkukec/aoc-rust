use regex::Regex;
#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let mut sum = 0;
    let re = Regex::new(r"\\x[0-9a-f]{2}").unwrap();
    for i in input.lines() {
        sum += i.len()
            - re.replace_all(i, "X")
                .replace("\\\\", "E")
                .replace("\\\"", "Q")
                .len()
            - 2;
    }
    sum
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    input.chars().filter(|x| *x == '"' || *x == '\\').count() + input.lines().count() * 2
}
