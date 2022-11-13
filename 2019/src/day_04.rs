use itertools::Itertools;
#[aoc_generator(day04)]
fn generate(input: &str) -> (u32, u32) {
    let (a, b) = input.split_once('-').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

#[aoc(day04, part1)]
pub fn part1(input: &(u32, u32)) -> u32 {
    let mut cnt = 0;
    for i in input.0..=input.1 {
        let mut i = i;
        let mut seen_two_same = false;
        let mut all_increasing = true;
        let mut last = 0;
        for n in 0..6 {
            let d = i % 10;
            if n != 0 {
                seen_two_same = seen_two_same || d == last;
                all_increasing = all_increasing && (d <= last);
            }
            i /= 10;
            last = d;
        }
        if seen_two_same && all_increasing {
            cnt += 1;
        }
    }
    cnt
}

#[aoc(day04, part2)]
pub fn part2(input: &(u32, u32)) -> i32 {
    let mut cnt = 0;
    for i in input.0..=input.1 {
        let mut i = i;
        let mut v = vec![11, 10]; // padding for seen_two_same, made so it wouldn't affect all_increasing
        for _ in 0..6 {
            let d = i % 10;
            v.push(d);
            i /= 10;
        }
        v.push(0); // more padding
        v.push(0);

        // it's reversed because i push the digits from least to most significant
        let all_increasing = v.iter().tuple_windows().all(|(x, y)| x >= y);

        let mut seen_two_same = false;

        for (a, b, c, d) in v.iter().tuple_windows() {
            if a != b && b == c && c != d {
                seen_two_same = true;
            }
        }

        if seen_two_same && all_increasing {
            cnt += 1;
        }
    }
    cnt
}
