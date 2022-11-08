use std::collections::HashSet;

type Point4D = (i32, i32, i32, i32);

fn manhattan(a: &Point4D, b: &Point4D) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1) + a.2.abs_diff(b.2) + a.3.abs_diff(b.3)
}

#[aoc_generator(day25)]
fn generate(input: &str) -> Vec<Point4D> {
    let mut out = Vec::with_capacity(input.lines().count());
    for l in input.lines() {
        let mut l = l.split(',');
        let a = l.next().unwrap().parse().unwrap();
        let b = l.next().unwrap().parse().unwrap();
        let c = l.next().unwrap().parse().unwrap();
        let d = l.next().unwrap().parse().unwrap();
        out.push((a, b, c, d));
    }

    out
}

// a simple recursive solution for the last day
fn get_group(seen: &mut HashSet<Point4D>, x: Point4D, points: &[Point4D]) {
    let new: Vec<_> = points
        .iter()
        .filter(|p| !seen.contains(p))
        .filter(|p| manhattan(p, &x) <= 3)
        .collect();
    seen.extend(new.iter().copied());
    new.iter()
        .copied()
        .for_each(|n| get_group(seen, *n, points));
}

#[aoc(day25, part1)]
pub fn part1(input: &[Point4D]) -> i32 {
    let mut input = input.to_vec();
    let mut cnt = 0;
    while let Some(x) = input.pop() {
        let mut g = HashSet::new();
        g.insert(x);
        get_group(&mut g, x, &input);
        input.retain(|p| !g.contains(p));
        cnt += 1;
    }
    cnt
}

#[aoc(day25, part2)]
pub fn part2(_input: &[Point4D]) -> &'static str {
    "ayy"
}
