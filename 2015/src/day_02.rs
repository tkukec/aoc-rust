#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|x| {
            let mut nums = x
                .split('x')
                .map(|i| i.parse().unwrap())
                .collect::<Vec<i32>>();
            nums.sort();
            3 * nums[0] * nums[1] + 2 * nums[1] * nums[2] + 2 * nums[0] * nums[2]
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|x| {
            let mut nums = x
                .split('x')
                .map(|i| i.parse().unwrap())
                .collect::<Vec<i32>>();
            nums.sort();
            2 * (nums[0] + nums[1]) + nums[0] * nums[1] * nums[2]
        })
        .sum()
}
