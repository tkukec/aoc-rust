use md5;
#[aoc(day4, part1)]
pub fn part1(input: &str) -> String {
    for i in (1..).map(|x| x.to_string()) {
        if format!("{:x}", md5::compute(input.to_owned() + &i)).starts_with("00000") {
            return i;
        }
    }
    unreachable!()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> String {
    for i in (1..).map(|x| x.to_string()) {
        if format!("{:x}", md5::compute(input.to_owned() + &i)).starts_with("000000") {
            return i;
        }
    }
    unreachable!()
}
