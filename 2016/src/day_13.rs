use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
struct Point(i32, i32);

fn not_wall(a: Point, fav_num: i32) -> bool {
    let (x, y) = (a.0, a.1);
    // x = -1 and y = -1 are walls, so you can't go into the negatives
    x >= 0 && y >= 0 && (x * x + 3 * x + 2 * x * y + y + y * y + fav_num).count_ones() % 2 == 0
}

fn dist(a: Point, b: Point) -> i32 {
    (if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 })
        + (if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 })
}

// A* implementation, algorithm from
// https://theory.stanford.edu/~amitp/GameProgramming/ImplementationNotes.html
fn search(start: Point, goal: Point, fav_num: i32) -> i32 {
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
        for (x_offset, y_offset) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let neighbor = Point(cur.0 + x_offset, cur.1 + y_offset);
            if not_wall(neighbor, fav_num) {
                let cost = cost_to[&cur] + 1;
                if !seen.contains(&neighbor) && q.get_priority(&neighbor).is_none() {
                    cost_to.insert(neighbor, cost);
                    q.push(neighbor, Reverse(cost + dist(neighbor, goal)));
                }
            }
        }
    }
    cost_to[&goal]
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> i32 {
    let goal = Point(31, 39);
    let start = Point(1, 1);
    let fav_num: i32 = input.parse().unwrap();
    assert!(not_wall(start, fav_num));

    search(start, goal, fav_num)
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> i32 {
    let start = Point(1, 1);
    let goal = Point(50, 50);
    let fav_num: i32 = input.parse().unwrap();

    let mut q = PriorityQueue::new();
    q.push(start, Reverse(1));

    let mut seen = HashSet::new();

    let mut cost_to = HashMap::new();
    cost_to.insert(start, 0);
    while let Some((cur, _)) = q.pop() {
        if cur.0 > 50 || cur.1 > 50 {
            continue;
        }
        seen.insert(cur);
        for (x_offset, y_offset) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let neighbor = Point(cur.0 + x_offset, cur.1 + y_offset);
            if not_wall(neighbor, fav_num) {
                let cost = cost_to[&cur] + 1;
                if !seen.contains(&neighbor) && q.get_priority(&neighbor).is_none() {
                    cost_to.insert(neighbor, cost);
                    q.push(neighbor, Reverse(cost + dist(neighbor, goal)));
                }
            }
        }
    }
    cost_to.values().filter(|cost| **cost <= 50).count() as i32
}
