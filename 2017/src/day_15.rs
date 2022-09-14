#[aoc(day15, part1)]
pub fn part1(input: &str) -> u32 {
    let mut a: u64 = input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let mut b: u64 = input
        .lines()
        .last()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let mut cnt = 0;
    for _ in 0..40000000 {
        a *= 16807;
        a %= 2147483647;
        b *= 48271;
        b %= 2147483647;

        if a << 48 == b << 48 {
            cnt += 1;
        }
    }
    cnt
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> i32 {
    let mut a: u64 = input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let mut b: u64 = input
        .lines()
        .last()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let mut cnt = 0;
    for _ in 0..5000000 {
        a *= 16807;
        a %= 2147483647;
        b *= 48271;
        b %= 2147483647;

        while a % 4 != 0 {
            a *= 16807;
            a %= 2147483647;
        }
        while b % 8 != 0 {
            b *= 48271;
            b %= 2147483647;
        }
        if a << 48 == b << 48 {
            cnt += 1;
        }
    }
    cnt
}
