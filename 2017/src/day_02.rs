#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let mut out = 0;
    for line in input.lines() {
        let mut l: Vec<u32> = line.split('\t').map(|x| x.parse().unwrap()).collect();
        l.sort();
        out += l[l.len() - 1] - l[0];
    }
    out
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    let mut out = 0;
    for line in input.lines() {
        let l: Vec<u32> = line.split('\t').map(|x| x.parse().unwrap()).collect();
        let x = *l
            .iter()
            .find(|&&x| l.iter().any(|&y| x != y && x % y == 0))
            .unwrap();
        let y = l.iter().find(|&&y| x != y && x % y == 0).unwrap();
        out += x / y;
    }
    out
}
