#[aoc(day20, part1)]
pub fn part1(input: &str) -> u32 {
    let mut input: u32 = input.parse().unwrap();
    input /= 10;
    // the result should probably be divisible by a highly composite number
    // i just tried it with 60 and the result was correct. without it, the search takes ~6 min
    // both answers are also divisible by another HCN, 10080, but i didn't know that
    (0u32..)
        .step_by(60)
        .find(|i| i + (1..=(i / 2 + 1)).filter(|j| i % j == 0).sum::<u32>() >= input)
        .unwrap()
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> u32 {
    let mut input: u32 = input.parse().unwrap();
    input /= 11;
    // the additional requirement actually helps make the code a bit faster
    (0u32..)
        .step_by(60)
        .skip(1) // first one tries to calc 0 % 0, that's not good
        .find(|i| i + ((i / 50)..=(i / 2 + 1)).filter(|j| i % j == 0).sum::<u32>() >= input)
        .unwrap()
}

// idea from https://www.reddit.com/r/adventofcode/comments/3xjpp2/comment/cy59zd9/
// not faster than the normal part 1, but it doesn't "cheat" by using only numbers divisible by 60
#[aoc(day20, part1, alternative)]
pub fn part1_2(input: &str) -> usize {
    let input: u32 = input.parse().unwrap();
    let mut houses: Vec<u32> = [0; 40000000].to_vec();
    (1u32..input).for_each(|i| {
        (i..input)
            .step_by(i as usize)
            .for_each(|j| houses[j as usize] += i * 10)
    });
    houses
        .into_iter()
        .enumerate()
        .find(|(_, y)| y >= &input)
        .unwrap()
        .0
}

// part 2 is much faster when using this approach
#[aoc(day20, part2, alternative)]
pub fn part2_2(input: &str) -> usize {
    let input: u32 = input.parse().unwrap();
    let mut houses: Vec<u32> = [0; 40000000].to_vec();
    (1u32..input).for_each(|i| {
        (i..input)
            .step_by(i as usize)
            .take(50)
            .for_each(|j| houses[j as usize] += i * 11)
    });
    houses
        .into_iter()
        .enumerate()
        .find(|(_, y)| y >= &input)
        .unwrap()
        .0
}
