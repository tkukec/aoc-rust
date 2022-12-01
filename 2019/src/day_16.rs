use std::iter::repeat;
fn apply_transform(a: Vec<i8>) -> Vec<i8> {
    a.iter()
        .enumerate()
        .map(|(i, _)| {
            let mut pat = repeat(
                repeat(0)
                    .take(i + 1)
                    .chain(repeat(1).take(i + 1))
                    .chain(repeat(0).take(i + 1))
                    .chain(repeat(-1).take(i + 1)),
            )
            .flatten();

            // skip 1st
            pat.next().unwrap();

            let res: i64 = a
                .iter()
                .zip(pat)
                .filter(|(_, b)| *b != 0)
                .map(|(a, b)| (a * b) as i64)
                .sum();

            (res.abs() % 10) as i8
        })
        .collect()
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> String {
    let mut input: Vec<i8> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i8)
        .collect();

    for _ in 0..100 {
        input = apply_transform(input);
    }

    input[0..8].iter().map(|x| x.to_string()).collect()
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> String {
    // the offset is 5 976 277, and the input is 6 500 000 long, so we just ignore everything
    // before the offset, because the pattern will be 0 to the offset, and then 1 to the end.
    // Only the sum is needed
    let offset: usize = input[..7].parse().unwrap();
    let mut input: Vec<u32> = repeat(input.chars().map(|x| x.to_digit(10).unwrap()))
        .take(10000)
        .flatten()
        .skip(offset)
        .collect();

    for _ in 0..100 {
        let sum: u32 = input.iter().sum();

        // scan over the input, the value is the sum from that element to the end
        input = input
            .iter()
            .scan(sum, |state, &x| {
                *state -= x;
                Some((*state + x) % 10)
            })
            .collect();
    }

    input[0..8].iter().map(|x| x.to_string()).collect()
}
