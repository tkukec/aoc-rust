use anyhow::Result;

use crate::intcode::exec_at;
use std::{
    collections::{HashSet, VecDeque},
    fmt::{Debug, Display},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    code: Vec<i64>,
    ptr: usize,
    rel_off: i64,
}

// returns only the new points
fn explore(
    a: Point,
    state: State,
    grid: &mut [[Tile; GRID_SIZE]; GRID_SIZE],
) -> Vec<(Point, Tile, State)> {
    let mut neigh = Vec::with_capacity(4);
    for d in [Dir::North, Dir::South, Dir::West, Dir::East] {
        let p = a.apply_dir(d);
        let gp = p.grid_pos();
        if grid[gp.1][gp.0] == Tile::Unknown {
            let (out, new_state) =
                run_until_output(d.into(), state.clone()).expect("End of program");
            let tile: Tile = out.try_into().unwrap();
            grid[gp.1][gp.0] = tile;
            neigh.push((p, tile, new_state));
        }
    }
    neigh
}

fn run_until_output(input: i64, mut state: State) -> Option<(i64, State)> {
    let mut input = VecDeque::from([input]);
    while let (Some(new_ptr), output) =
        exec_at(&mut state.code, state.ptr, &mut input, &mut state.rel_off)
    {
        state.ptr = new_ptr;
        if let Some(output) = output {
            return Some((output, state));
        }
    }

    None
}

const GRID_SIZE: usize = 50;
const HALF_GRID: i32 = GRID_SIZE as i32 / 2;

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    let code: Vec<i64> = input
        .split(',')
        .map(|x| x.parse().unwrap())
        //.chain([0; 1000].into_iter())
        .collect();

    let mut grid = [[Tile::Unknown; GRID_SIZE]; GRID_SIZE];
    let state = State {
        code,
        ptr: 0,
        rel_off: 0,
    };
    let mut q = VecDeque::new();
    q.push_back((Point { x: 0, y: 0 }, state, 0));
    while let Some((p, s, c)) = q.pop_front() {
        let res = explore(p, s, &mut grid);
        for x in res
            .into_iter()
            .filter(|(_, x, _)| matches!(x, Tile::Empty | Tile::Goal))
        {
            if x.1 == Tile::Goal {
                return c + 1;
            } else {
                q.push_back((x.0, x.2, c + 1));
            }
        }
    }

    unreachable!();
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> i64 {
    let code: Vec<i64> = input
        .split(',')
        .map(|x| x.parse().unwrap())
        //.chain([0; 1000].into_iter())
        .collect();

    let mut grid = [[Tile::Unknown; GRID_SIZE]; GRID_SIZE];
    let state = State {
        code,
        ptr: 0,
        rel_off: 0,
    };
    let mut q = VecDeque::new();
    q.push_back((Point { x: 0, y: 0 }, state, 0));
    let mut goal_pos: Option<Point> = None;

    while let Some((p, s, c)) = q.pop_front() {
        let res = explore(p, s, &mut grid);
        for x in res
            .into_iter()
            .filter(|(_, x, _)| matches!(x, Tile::Empty | Tile::Goal))
        {
            if x.1 == Tile::Goal {
                println!("goal at {}", x.0);
                goal_pos = Some(x.0);
            }
            q.push_back((x.0, x.2, c + 1));
        }
    }
    let goal_pos = goal_pos.expect("Goal not found");

    let mut seen = HashSet::new();

    let mut q = VecDeque::new();
    q.push_back((goal_pos, 0));

    let mut max = 0;

    while let Some((pt, c)) = q.pop_front() {
        for d in [Dir::North, Dir::South, Dir::West, Dir::East] {
            let n = pt.apply_dir(d);
            if !seen.contains(&n) {
                let ng = n.grid_pos();
                if grid[ng.1][ng.0] == Tile::Empty {
                    q.push_back((n, c + 1));
                    max = std::cmp::max(max, c + 1);
                    seen.insert(n);
                }
            }
        }
    }

    max
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Unknown,
    Wall,
    Empty,
    Goal,
}

impl TryFrom<i64> for Tile {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Tile::Wall),
            1 => Ok(Tile::Empty),
            2 => Ok(Tile::Goal),
            _ => Err("Bad tile id"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl From<Dir> for i64 {
    fn from(d: Dir) -> Self {
        match d {
            Dir::North => 1,
            Dir::South => 2,
            Dir::West => 3,
            Dir::East => 4,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn apply_dir(self, dir: Dir) -> Self {
        match dir {
            Dir::North => (self.x - 1, self.y).into(),
            Dir::South => (self.x + 1, self.y).into(),
            Dir::West => (self.x, self.y - 1).into(),
            Dir::East => (self.x, self.y + 1).into(),
        }
    }
    fn grid_pos(self) -> (usize, usize) {
        (
            // panics only if the grid is not big enough to store the data
            (self.x + HALF_GRID).try_into().unwrap(),
            (self.y + HALF_GRID).try_into().unwrap(),
        )
    }
}

impl From<(i32, i32)> for Point {
    fn from(i: (i32, i32)) -> Self {
        Point { x: i.0, y: i.1 }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
}
