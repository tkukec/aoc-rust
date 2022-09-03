#[aoc(day20, part1)]
pub fn part1(input: &str) -> u32 {
    let mut pairs = vec![];
    for x in input.lines() {
        let start: u32 = x.split('-').next().unwrap().parse().unwrap();
        let end: u32 = x.split('-').nth(1).unwrap().parse().unwrap();
        pairs.push((start, end));
    }
    pairs.sort();
    let mut x = 0;
    while let Some((_, b)) = pairs.iter().find(|(s, e)| (s..=e).contains(&&x)) {
        x = b + 1
    }
    x
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> u32 {
    let mut pairs = vec![];
    for x in input.lines() {
        let start: u32 = x.split('-').next().unwrap().parse().unwrap();
        let end: u32 = x.split('-').nth(1).unwrap().parse().unwrap();
        pairs.push((start, end));
    }
    pairs.sort();
    let mut cnt = 0;
    let mut x = 0;
    while x != u32::MAX {
        if let Some((_, e)) = pairs.iter().find(|(s, e)| (s..=e).contains(&&x)) {
            x = e.saturating_add(1); // if e == u32::MAX, x overflows back to 0
        } else {
            x += 1;
            cnt += 1;
        }
    }
    cnt
}
