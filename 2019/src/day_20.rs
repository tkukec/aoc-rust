use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

// the Portal tile itself doesn't contain any data,
// it's just a marker for data stored in a separate hashmap
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Tile {
    Void,
    Empty,
    Wall,
    Portal,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            _ => Tile::Void,
        }
    }
}

type Point = (usize, usize);

#[derive(Clone)]
pub struct Puzzle {
    start: Point,
    end: Point,
    grid: Vec<Vec<Tile>>,
    links: HashMap<Point, Point>,
}

#[aoc_generator(day20)]
fn generate(input: &str) -> Puzzle {
    let grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let mut out_grid: Vec<Vec<Tile>> = grid
        .iter()
        .map(|x| x.iter().map(|&p| p.into()).collect())
        .collect();

    let mut all_portals: HashSet<PortalData> = HashSet::new();
    type PortalData = (Point, (char, char));
    for (i, (&x, &y)) in grid[0].iter().zip(grid[1].iter()).enumerate() {
        if x != ' ' {
            all_portals.insert(((i, 2), (x, y)));
        }
    }
    for (i, (&x, &y)) in grid[grid.len() - 2]
        .iter()
        .zip(grid.last().unwrap().iter())
        .enumerate()
    {
        if x != ' ' {
            all_portals.insert(((i, grid.len() - 3), (x, y)));
        }
    }
    for (i, (x, y)) in grid
        .iter()
        .map(|x| x[0])
        .zip(grid.iter().map(|x| x[1]))
        .enumerate()
    {
        if x != ' ' {
            all_portals.insert(((2, i), (x, y)));
        }
    }
    for (i, (x, y)) in grid
        .iter()
        .map(|x| x[x.len() - 2])
        .zip(grid.iter().map(|x| *x.last().unwrap()))
        .enumerate()
    {
        if x != ' ' {
            all_portals.insert(((grid[0].len() - 3, i), (x, y)));
        }
    }

    let inner_top = grid
        .iter()
        .enumerate()
        .filter(|(_, x)| x.windows(3).any(|x| x == [' ', ' ', ' ']))
        .nth(2)
        .unwrap()
        .0;
    let inner_bottom = grid
        .iter()
        .enumerate()
        .filter(|(_, x)| x.windows(3).any(|x| x == [' ', ' ', ' ']))
        .nth_back(2)
        .unwrap()
        .0;
    let inner_left = grid[inner_top]
        .windows(3)
        .position(|x| x == [' ', ' ', ' '])
        .unwrap();
    let inner_right = grid[0].len()
        - grid[inner_top]
            .windows(3)
            .rev()
            .position(|x| x == [' ', ' ', ' '])
            .unwrap()
        - 1;

    // println!("{inner_top}, {inner_bottom}, {inner_left}, {inner_right}");

    for (i, (&x, &y)) in grid[inner_top][inner_left..=inner_right]
        .iter()
        .zip(grid[inner_top + 1][inner_left..=inner_right].iter())
        .enumerate()
    {
        if x != ' ' {
            all_portals.insert(((i + inner_left, inner_top - 1), (x, y)));
        }
    }
    for (i, (&x, &y)) in grid[inner_bottom - 1][inner_left..=inner_right]
        .iter()
        .zip(grid[inner_bottom][inner_left..=inner_right].iter())
        .enumerate()
    {
        if x != ' ' {
            all_portals.insert(((i + inner_left, inner_bottom + 1), (x, y)));
        }
    }
    for (i, (x, y)) in grid[inner_top..=inner_bottom]
        .iter()
        .map(|x| x[inner_left])
        .zip(
            grid[inner_top..=inner_bottom]
                .iter()
                .map(|x| x[inner_left + 1]),
        )
        .enumerate()
    {
        if x != ' ' {
            all_portals.insert(((inner_left - 1, i + inner_top), (x, y)));
        }
    }
    for (i, (x, y)) in grid[inner_top..=inner_bottom]
        .iter()
        .map(|x| x[inner_right - 1])
        .zip(
            grid[inner_top..=inner_bottom]
                .iter()
                .map(|x| x[inner_right]),
        )
        .enumerate()
    {
        if x != ' ' {
            all_portals.insert(((inner_right + 1, i + inner_top), (x, y)));
        }
    }
    for ((x, y), _) in all_portals.iter().copied() {
        assert_eq!(grid[y][x], '.');
    }

    let start = all_portals.iter().find(|x| x.1 == ('A', 'A')).unwrap().0;
    let end = all_portals.iter().find(|x| x.1 == ('Z', 'Z')).unwrap().0;
    all_portals
        .iter()
        .copied()
        .for_each(|((x, y), _)| out_grid[y][x] = Tile::Portal);
    let mut links: HashMap<Point, Point> = HashMap::new();
    for [a, b] in all_portals
        .iter()
        .copied()
        .sorted_by_key(|x| x.1)
        .skip(1) // skip the AA portal, ZZ is discarded by array_chunks
        .array_chunks()
    {
        assert_eq!(a.1, b.1);
        links.insert(a.0, b.0);
        links.insert(b.0, a.0);
    }
    Puzzle {
        start,
        end,
        grid: out_grid,
        links,
    }
}

#[aoc(day20, part1)]
pub fn part1(input: &Puzzle) -> i32 {
    let Puzzle {
        start,
        end,
        grid,
        links,
    } = input.clone();

    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    while let Some((p, len)) = q.pop_front() {
        if p == end {
            return len - 1;
        }
        if seen.contains(&p) {
            continue;
        }
        seen.insert(p);

        for x in get_neigh(p, &grid)
            .into_iter()
            .filter(|x| !seen.contains(x))
        {
            match grid[x.1][x.0] {
                Tile::Portal => q.push_back((*links.get(&x).unwrap_or(&x), len + 2)),
                Tile::Empty => q.push_back((x, len + 1)),
                _ => panic!("Invalid position"),
            }
        }
    }

    panic!("No path found")
}

fn get_neigh(a: Point, grid: &[Vec<Tile>]) -> Vec<Point> {
    let mut out = Vec::with_capacity(4);
    if matches!(grid[a.1 + 1][a.0], Tile::Empty | Tile::Portal) {
        out.push((a.0, a.1 + 1))
    }
    if matches!(grid[a.1 - 1][a.0], Tile::Empty | Tile::Portal) {
        out.push((a.0, a.1 - 1))
    }
    if matches!(grid[a.1][a.0 + 1], Tile::Empty | Tile::Portal) {
        out.push((a.0 + 1, a.1))
    }
    if matches!(grid[a.1][a.0 - 1], Tile::Empty | Tile::Portal) {
        out.push((a.0 - 1, a.1))
    }
    out
}

#[aoc(day20, part2)]
pub fn part2(input: &Puzzle) -> i32 {
    let Puzzle {
        start,
        end,
        grid,
        links,
    } = input.clone();

    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((start, 0, 0));
    while let Some((p, lvl, len)) = q.pop_front() {
        if seen.contains(&(p, lvl)) {
            continue;
        }
        seen.insert((p, lvl));

        for n in get_neigh(p, &grid)
            .into_iter()
            .filter(|n| !seen.contains(&(*n, lvl)))
        {
            if n == end {
                if lvl == 0 {
                    return len + 1;
                } else {
                    continue;
                }
            }

            if n == start && lvl != 0 {
                continue;
            }
            match grid[n.1][n.0] {
                Tile::Portal => q.push_back((
                    *links.get(&n).unwrap(),
                    match n {
                        (_x @ 25..=100, _y @ 25..=100) => lvl + 1,
                        _ => {
                            if lvl > 0 {
                                lvl - 1
                            } else {
                                continue;
                            }
                        }
                    },
                    len + 2,
                )),
                Tile::Empty => q.push_back((n, lvl, len + 1)),
                _ => panic!("Invalid position"),
            }
        }
    }

    panic!("No path found")
}
