use itertools::Itertools;
const GRID_SIZE: usize = 400;
#[aoc_generator(day06)]
fn generate(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|x| x.split_once(", ").unwrap())
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .collect()
}

fn dist(a: (u32, u32), b: (u32, u32)) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

#[aoc(day06, part1)]
pub fn part1(input: &[(u32, u32)]) -> usize {
    let mut grid = [[0_u32; GRID_SIZE]; GRID_SIZE];
    for (i, l) in grid.iter_mut().enumerate() {
        for (j, c) in l.iter_mut().enumerate() {
            let pt = (j as u32, i as u32);
            let closest: Vec<(usize, (u32, u32))> = input
                .iter()
                .copied()
                .enumerate()
                .sorted_by_key(|x| dist(x.1, pt))
                .take(2)
                .collect();
            if dist(closest[0].1, pt) != dist(closest[1].1, pt) {
                *c = closest[0].0 as u32;
            } else {
                *c = u32::MAX;
            }
        }
    }
    let a: Vec<u32> = grid.iter().copied().flatten().collect();
    let best = *a
        .iter()
        .filter(|x| {
            !grid[0].contains(x)
                && !grid[GRID_SIZE - 1].contains(x)
                && grid.iter().all(|l| l[0] != **x && l[GRID_SIZE - 1] != **x)
        })
        .max_by_key(|x| a.iter().filter(|n| n == x).count())
        .unwrap();

    a.iter().filter(|g| **g == best).count()
}

#[aoc(day06, part2)]
pub fn part2(input: &[(u32, u32)]) -> u32 {
    let mut cnt: u32 = 0;
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let pt = (j as u32, i as u32);
            let dist_all: u32 = input.iter().copied().map(|x| dist(x, pt)).sum();
            if dist_all < 10000 {
                cnt += 1;
            }
        }
    }
    cnt
}
