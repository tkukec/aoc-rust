#[derive(Clone, Copy, Debug)]
struct Disk {
    p: u32,
    cur: u32,
}

fn generate(input: &str) -> Vec<Disk> {
    let mut out = vec![];
    for x in input.lines() {
        let mut words = x.split(' ');
        let num = words.nth(3).unwrap().parse().unwrap();
        let cur = words.nth(7).unwrap().trim_end_matches('.').parse().unwrap();
        out.push(Disk { p: num, cur });
    }
    out
}
#[aoc(day15, part1)]
pub fn part1(input: &str) -> u32 {
    let disks = generate(input);
    let mut i = 0;
    let mut good = false;
    while !good {
        if (0..disks.len()).all(|n| (i + disks[n].cur + n as u32 + 1) % disks[n].p == 0) {
            good = true;
        } else {
            i += 1;
        }
    }
    i
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> u32 {
    let mut disks = generate(input);
    disks.push(Disk { p: 11, cur: 0 });
    let mut i = 0;
    let mut good = false;
    while !good {
        if (0..disks.len()).all(|n| (i + disks[n].cur + n as u32 + 1) % disks[n].p == 0) {
            good = true;
        } else {
            i += 1;
        }
    }
    i
}
