#[aoc(day4, part1)]
pub fn part1(input: &str) -> String {
    for i in (1..).map(|x| x.to_string()) {
        // how i originally solved both parts, pt. 2 takes ~6 seconds
        if format!("{:x}", md5::compute(input.to_owned() + &i)).starts_with("00000") {
            return i;
        }
    }
    unreachable!()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> String {
    for i in (1..).map(|x| x.to_string()) {
        let a = md5::compute(input.to_owned() + &i);
        // https://gist.github.com/gkbrk/2e4835e3a17b3fb6e1e7
        if a[..3] == [0, 0, 0] {
            return i;
        }
    }
    unreachable!()
}
