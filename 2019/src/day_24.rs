use std::collections::{HashMap, HashSet};

const T: bool = true;
const F: bool = false;

// can easily parse it into a vec, couldn't care enough to parse it into a grid
const GRID: [[bool; 5]; 5] = [
    [T, T, T, F, F],
    [F, T, T, F, F],
    [T, F, F, F, F],
    [T, T, F, F, T],
    [F, T, T, T, F],
];
type Point = (usize, usize);
fn get_neigh(pt: Point, grid: &[[bool; 5]]) -> u8 {
    grid.get(pt.1)
        .map(|x| {
            *x.get(pt.0 + 1).unwrap_or(&false) as u8 + *x.get(pt.0 - 1).unwrap_or(&false) as u8
        })
        .unwrap_or(0)
        + grid.get(pt.1 + 1).map(|x| x[pt.0] as u8).unwrap_or(0)
        + grid.get(pt.1 - 1).map(|x| x[pt.0] as u8).unwrap_or(0)
}

fn biodiversity(grid: &[[bool; 5]]) -> usize {
    grid.iter()
        .flatten()
        .zip((0..).map(|x| 1usize << x))
        .map(|(pt, n)| if *pt { n } else { 0 })
        .sum()
}

#[aoc(day24, part1)]
pub fn part1(_input: &str) -> usize {
    let mut grid = GRID;
    let mut seen = HashSet::new();
    seen.insert(grid);
    loop {
        let old_grid = grid;
        for y in 0..5 {
            for x in 0..5 {
                let cur = old_grid[y][x];
                let neigh_count = get_neigh((x, y), &old_grid);
                //println!("{x} {y} {neigh_count}");
                if cur && neigh_count != 1 {
                    grid[y][x] = false;
                } else if !cur && (1..=2).contains(&neigh_count) {
                    grid[y][x] = true;
                }
            }
        }
        if seen.contains(&grid) {
            return biodiversity(&grid);
        } else {
            seen.insert(grid);
        }
    }
}

fn get_neigh2(pt: Point, lvl: i32, levels: &HashMap<i32, [[bool; 5]; 5]>) -> u8 {
    let (x, y) = pt;

    //      |     |         |     |
    //   1  |  2  |    3    |  4  |  5
    //      |     |         |     |
    // -----+-----+---------+-----+-----
    //      |     |         |     |
    //   6  |  7  |    8    |  9  |  10
    //      |     |         |     |
    // -----+-----+---------+-----+-----
    //      |     |A|B|C|D|E|     |
    //      |     |-+-+-+-+-|     |
    //      |     |F|G|H|I|J|     |
    //      |     |-+-+-+-+-|     |
    //  11  | 12  |K|L|?|N|O|  14 |  15
    //      |     |-+-+-+-+-|     |
    //      |     |P|Q|R|S|T|     |
    //      |     |-+-+-+-+-|     |
    //      |     |U|V|W|X|Y|     |
    // -----+-----+---------+-----+-----
    //      |     |         |     |
    //  16  | 17  |    18   |  19 |  20
    //      |     |         |     |
    // -----+-----+---------+-----+-----
    //      |     |         |     |
    //  21  | 22  |    23   |  24 |  25
    //      |     |         |     |
    let cur = levels[&lvl];
    match (x, y) {
        (0, 0) => {
            // A
            levels
                .get(&(lvl - 1))
                .map(|x| x[1][2] as u8 + x[2][1] as u8)
                .unwrap_or(0)
                + cur[0][1] as u8
                + cur[1][0] as u8
        }
        (4, 0) => {
            // E
            levels
                .get(&(lvl - 1))
                .map(|x| x[1][2] as u8 + x[2][3] as u8)
                .unwrap_or(0)
                + cur[0][3] as u8
                + cur[1][4] as u8
        }
        (0, 4) => {
            // U
            levels
                .get(&(lvl - 1))
                .map(|x| x[3][2] as u8 + x[2][1] as u8)
                .unwrap_or(0)
                + cur[3][0] as u8
                + cur[4][1] as u8
        }
        (4, 4) => {
            // Y
            levels
                .get(&(lvl - 1))
                .map(|x| x[3][2] as u8 + x[2][3] as u8)
                .unwrap_or(0)
                + cur[3][4] as u8
                + cur[4][3] as u8
        }
        (2, 1) => {
            // H
            levels
                .get(&(lvl + 1))
                .map(|x| x[0].iter().map(|x| *x as u8).sum())
                .unwrap_or(0)
                + cur[0][2] as u8
                + cur[1][1] as u8
                + cur[1][3] as u8
        }
        (3, 2) => {
            // N
            levels
                .get(&(lvl + 1))
                .map(|x| x.iter().map(|x| x[4] as u8).sum())
                .unwrap_or(0)
                + cur[1][3] as u8
                + cur[2][4] as u8
                + cur[3][3] as u8
        }
        (2, 3) => {
            // R
            levels
                .get(&(lvl + 1))
                .map(|x| x[4].iter().map(|x| *x as u8).sum())
                .unwrap_or(0)
                + cur[4][2] as u8
                + cur[3][1] as u8
                + cur[3][3] as u8
        }
        (1, 2) => {
            // L
            levels
                .get(&(lvl + 1))
                .map(|x| x.iter().map(|x| x[0] as u8).sum())
                .unwrap_or(0)
                + cur[2][0] as u8
                + cur[1][1] as u8
                + cur[3][1] as u8
        }
        (_, 0) => {
            // BCD
            levels.get(&(lvl - 1)).map(|l| l[1][2] as u8).unwrap_or(0)
                + cur[0][x - 1] as u8
                + cur[0][x + 1] as u8
                + cur[1][x] as u8
        }
        (_, 4) => {
            // VWX
            levels.get(&(lvl - 1)).map(|l| l[3][2] as u8).unwrap_or(0)
                + cur[4][x - 1] as u8
                + cur[4][x + 1] as u8
                + cur[3][x] as u8
        }
        (0, _) => {
            // FKP
            levels.get(&(lvl - 1)).map(|l| l[2][1] as u8).unwrap_or(0)
                + cur[y - 1][0] as u8
                + cur[y + 1][0] as u8
                + cur[y][1] as u8
        }
        (4, _) => {
            // JOT
            levels.get(&(lvl - 1)).map(|l| l[2][3] as u8).unwrap_or(0)
                + cur[y - 1][4] as u8
                + cur[y + 1][4] as u8
                + cur[y][3] as u8
        }
        _ => {
            // GIQS
            get_neigh(pt, &cur)
        }
    }
}

#[aoc(day24, part2)]
pub fn part2(_input: &str) -> usize {
    let mut levels = HashMap::from([(0, GRID)]);

    for i in 1..=200 {
        levels.insert(i, [[false; 5]; 5]);
        levels.insert(-i, [[false; 5]; 5]);

        let old_lvl = levels.clone();
        for (lvl, old_grid) in old_lvl.iter() {
            for (y, l) in old_grid.iter().enumerate() {
                for (x, p) in l.iter().enumerate() {
                    if x == 2 && y == 2 {
                        continue;
                    }
                    let cur = *p;
                    let neigh_count = get_neigh2((x, y), *lvl, &old_lvl);
                    // println!("{lvl} {x} {y} {neigh_count}");
                    if cur && neigh_count != 1 {
                        levels.get_mut(lvl).unwrap()[y][x] = false;
                    } else if !cur && (1..=2).contains(&neigh_count) {
                        levels.get_mut(lvl).unwrap()[y][x] = true;
                    }
                }
            }
        }
    }

    levels.values().flatten().flatten().filter(|x| **x).count()
}
