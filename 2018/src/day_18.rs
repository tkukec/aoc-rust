use itertools::Itertools;
use std::collections::HashMap;

const SIZE: usize = 50;
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Acre {
    Lumberyard,
    Open,
    Wooded,
}

impl From<char> for Acre {
    fn from(c: char) -> Self {
        match c {
            '|' => Acre::Wooded,
            '#' => Acre::Lumberyard,
            '.' => Acre::Open,
            _ => panic!("Bad input"),
        }
    }
}

#[aoc_generator(day18)]
fn generate(input: &str) -> Vec<Vec<Acre>> {
    input
        .lines()
        .map(|x| x.chars().map(Acre::from).collect())
        .collect()
}

fn neighbours(grid: &[Vec<Acre>], coord: (usize, usize)) -> HashMap<Acre, u8> {
    let (x, y) = coord;
    let mut neighbours = HashMap::from([(Acre::Open, 0), (Acre::Wooded, 0), (Acre::Lumberyard, 0)]);
    let offsets = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    for (x_off, y_off) in offsets {
        if let Some(Some(pt)) = grid
            .get((y as i32 + y_off) as usize)
            .map(|l: &Vec<Acre>| l.get((x as i32 + x_off) as usize))
        {
            neighbours.entry(*pt).and_modify(|x| *x += 1);
        }
    }
    neighbours
}

fn gen_next(grid: &[Vec<Acre>]) -> Vec<Vec<Acre>> {
    let mut new = grid.to_owned();

    for y in 0..SIZE {
        for x in 0..SIZE {
            let cur = grid[y][x];
            let n = neighbours(grid, (x, y));
            if cur == Acre::Open && n[&Acre::Wooded] >= 3 {
                new[y][x] = Acre::Wooded;
            } else if cur == Acre::Wooded && n[&Acre::Lumberyard] >= 3 {
                new[y][x] = Acre::Lumberyard;
            } else if cur == Acre::Lumberyard
                && !(n[&Acre::Lumberyard] >= 1 && n[&Acre::Wooded] >= 1)
            {
                new[y][x] = Acre::Open;
            }
        }
    }
    new
}

#[aoc(day18, part1)]
pub fn part1(input: &[Vec<Acre>]) -> usize {
    let mut grid = input.to_owned();
    for _ in 0..10 {
        grid = gen_next(&grid);
    }

    let a = grid.iter().flatten().counts();
    a[&Acre::Wooded] * a[&Acre::Lumberyard]
}

#[aoc(day18, part2)]
pub fn part2(input: &[Vec<Acre>]) -> usize {
    let mut seen: Vec<Vec<Vec<Acre>>> = vec![];
    let mut grid = input.to_owned();
    seen.push(grid.clone());
    loop {
        grid = gen_next(&grid);

        if seen.contains(&grid) {
            let loop_start = seen.iter().position(|x| x == &grid).unwrap();
            let loop_end = seen.len();
            let loop_len = loop_end - loop_start;
            let loop_left = (1000000000 - loop_start) % loop_len;
            let last = &seen[loop_start + loop_left];

            let a = last.iter().flatten().counts();
            return a[&Acre::Wooded] * a[&Acre::Lumberyard];
        }

        seen.push(grid.clone());
    }
}
