use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use priority_queue::PriorityQueue;

use itertools::Itertools;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Point(u32, u32);

#[allow(clippy::upper_case_acronyms)]
#[derive(Hash, PartialEq, Clone, Copy)]
enum PointType {
    Wall,
    Path,
    POI(u8),
}

impl PointType {
    fn is_not_wall(&self) -> bool {
        self != &PointType::Wall
    }
}

impl Point {
    fn get_neighbors(&self, graph: &HashMap<Point, PointType>) -> Vec<Point> {
        let mut a = vec![];
        if graph[&Point(self.0 - 1, self.1)].is_not_wall() {
            a.push(Point(self.0 - 1, self.1));
        }
        if graph[&Point(self.0 + 1, self.1)].is_not_wall() {
            a.push(Point(self.0 + 1, self.1));
        }
        if graph[&Point(self.0, self.1 - 1)].is_not_wall() {
            a.push(Point(self.0, self.1 - 1));
        }
        if graph[&Point(self.0, self.1 + 1)].is_not_wall() {
            a.push(Point(self.0, self.1 + 1));
        }
        a
    }
}

fn generate(input: &str) -> HashMap<Point, PointType> {
    let mut grid = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            grid.insert(
                Point(x as u32, y as u32),
                match chr {
                    '#' => PointType::Wall,
                    '.' => PointType::Path,
                    x if x.is_ascii_digit() => PointType::POI(x.to_digit(10).unwrap() as u8),
                    _ => unreachable!(),
                },
            );
        }
    }

    grid
}

// Manhattan distance between two points
fn h_dist(a: Point, b: Point) -> u32 {
    (if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 })
        + (if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 })
}

// A* implementation, algorithm from
// https://theory.stanford.edu/~amitp/GameProgramming/ImplementationNotes.html
fn shortest_dist(start: Point, goal: Point, grid: &HashMap<Point, PointType>) -> u32 {
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
        for neighbor in cur.get_neighbors(grid) {
            let cost = cost_to[&cur] + 1;
            if !seen.contains(&neighbor) && q.get_priority(&neighbor).is_none() {
                cost_to.insert(neighbor, cost);
                q.push(neighbor, Reverse(cost + h_dist(neighbor, goal)));
            }
        }
    }
    cost_to[&goal]
}

fn shortest_dist_graph(
    grid: &HashMap<Point, PointType>,
    poi_list: Vec<Point>,
) -> HashMap<(Point, Point), u32> {
    let mut dist_graph = HashMap::new();
    for &x in poi_list.iter() {
        for &y in poi_list.iter() {
            if x != y && dist_graph.get(&(x, y)).is_none() {
                let val = shortest_dist(x, y, grid);
                dist_graph.insert((x, y), val);
                dist_graph.insert((y, x), val);
            }
        }
    }
    dist_graph
}

// Brute force solution, generates every path of length 8, and calculates the best score.
// There are only 8! = 40320 permutations, so optimizing isn't necessary
fn shortest_path_all_visited(part_2: bool, grid: HashMap<Point, PointType>) -> u32 {
    let graph = shortest_dist_graph(
        &grid,
        grid.iter()
            .filter(|(_, x)| matches!(x, PointType::POI(_)))
            .map(|(x, _)| *x)
            .collect(),
    );

    let starting_node = grid
        .iter()
        .find_map(|(&key, &val)| {
            if val == PointType::POI(0) {
                Some(key)
            } else {
                None
            }
        })
        .unwrap();

    let mut all_nodes = graph.keys().map(|(x, _)| *x).collect::<HashSet<Point>>();
    all_nodes.remove(&starting_node);
    let mut best = u32::MAX;

    for mut order in all_nodes.iter().permutations(7) {
        order.insert(0, &starting_node);
        if part_2 {
            order.push(&starting_node);
        }

        let score = order.windows(2).map(|x| graph[&(*x[0], *x[1])]).sum();
        best = std::cmp::min(best, score);
    }
    best
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> u32 {
    let grid = generate(input);
    shortest_path_all_visited(false, grid)
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> u32 {
    let grid = generate(input);
    shortest_path_all_visited(true, grid)
}
