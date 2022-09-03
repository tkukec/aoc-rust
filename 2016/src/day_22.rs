use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use priority_queue::PriorityQueue;

const X_MAX: u32 = 33;
const Y_MAX: u32 = 29;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point(u32, u32);

struct Node {
    used: u32,
    size: u32,
}

impl Node {
    fn avail(&self) -> u32 {
        self.size - self.used
    }
    fn is_empty(&self) -> bool {
        self.used == 0
    }
}

impl Point {
    fn get_neighbors(&self) -> Vec<Point> {
        let mut a = vec![];
        if self.0 > 0 {
            a.push(Point(self.0 - 1, self.1));
        }
        if self.0 < X_MAX {
            a.push(Point(self.0 + 1, self.1));
        }
        if self.1 > 0 {
            a.push(Point(self.0, self.1 - 1));
        }
        if self.1 < Y_MAX {
            a.push(Point(self.0, self.1 + 1));
        }
        a
    }
}

fn generate(input: &str) -> HashMap<Point, Node> {
    let mut grid = HashMap::new();
    for line in input.lines().skip(2) {
        let mut line = line.chars();
        let x: u32 = (&mut line)
            .skip(16)
            .take_while(|x| x.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap();
        let y: u32 = (&mut line)
            .skip(1)
            .take_while(|x| x.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap();
        let size: u32 = (&mut line)
            .skip_while(|x| *x == ' ')
            .take_while(|x| x.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap();
        let used: u32 = line
            .skip(1)
            .skip_while(|x| *x == ' ')
            .take_while(|x| x.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap();
        grid.insert(Point(x, y), Node { size, used });
    }

    grid
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> i32 {
    let grid = generate(input);
    let mut cnt = 0;
    for p in grid.keys() {
        for n in grid.keys().filter(|n| *n != p) {
            if grid[n].avail() >= grid[p].used && !grid[p].is_empty() {
                cnt += 1;
            }
        }
    }
    cnt
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Used,
    Wall,
}

// Manhattan distance between two points
fn h_dist(a: Point, b: Point) -> u32 {
    (if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 })
        + (if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 })
}

// A* implementation, algorithm from
// https://theory.stanford.edu/~amitp/GameProgramming/ImplementationNotes.html
fn shortest_dist(start: Point, goal: Point, grid: HashMap<Point, Tile>) -> u32 {
    let mut q = PriorityQueue::new();
    q.push(start, Reverse(1));

    let mut seen = HashSet::new();

    let mut cost_to = HashMap::new();
    cost_to.insert(start, 0);
    while let Some((cur, _)) = q.pop() {
        if cur == goal {
            break;
        }
        seen.insert(cur);
        for neighbor in cur.get_neighbors() {
            if grid[&neighbor] != Tile::Wall {
                let cost = cost_to[&cur] + 1;
                if !seen.contains(&neighbor) && q.get_priority(&neighbor).is_none() {
                    cost_to.insert(neighbor, cost);
                    q.push(neighbor, Reverse(cost + h_dist(neighbor, goal)));
                }
            }
        }
    }
    cost_to[&goal]
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> u32 {
    let grid = generate(input);
    let new_grid = HashMap::from_iter(grid.into_iter().map(|(p, v)| {
        if v.used > 89 {
            (p, Tile::Wall)
        } else if v.is_empty() {
            (p, Tile::Empty)
        } else {
            (p, Tile::Used)
        }
    }));

    let empty = *new_grid.iter().find(|(_, x)| **x == Tile::Empty).unwrap().0;
    let spot_before_goal = Point(X_MAX - 1, 0);

    // The total moves required are the sum of:
    // - distance from the empty spot to (32, 0), while avoiding walls
    // - 1 normal move (33, 0) -> (32, 0)
    // - 32 sliding maneuvers, each of which requires 5 moves
    //    0     1     2     3     4     5
    //   .G_ | .G. | .G. | .G. | _G. | G_.
    //   ... | .._ | ._. | _.. | ... | ...
    shortest_dist(empty, spot_before_goal, new_grid) + 1 + (X_MAX - 1) * 5
}
