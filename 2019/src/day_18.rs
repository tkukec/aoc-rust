use priority_queue::PriorityQueue;
use std::{
    cmp::Reverse,
    collections::{HashMap, VecDeque},
    fmt::Debug,
};

use itertools::Itertools;

type Point = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Key {
    val: u8,
}

const START: Key = Key { val: 255 };

impl Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if *self == START {
            write!(f, "key-@")
        } else {
            write!(f, "key-{}", (self.val + b'a') as char)
        }
    }
}
impl From<char> for Key {
    fn from(val: char) -> Self {
        if val == '@' {
            START
        } else if val.is_ascii_lowercase() {
            Key {
                val: (val as u8) - b'a',
            }
        } else {
            panic!("Invalid key character {val}");
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Keys {
    val: u32,
}

impl Keys {
    fn add_key(&mut self, key: Key) {
        if key != START {
            if self.has_key(key) {
                panic!("Keys {self:?} already have key {key:?}");
            }
            self.val |= 1 << key.val
        }
    }

    fn has_key(&self, key: Key) -> bool {
        self.val & 1 << key.val != 0
    }

    fn can_cross(&self, e: Edge) -> bool {
        (self.val & e.keys_needed.val) == e.keys_needed.val
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Edge {
    keys_needed: Keys,
    len: u32,
}

impl Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Edge\n{:?}\nlen = {}", self.keys_needed, self.len)
    }
}

impl Debug for Keys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\nZYXWVUTSRQPONMLKJIHGFEDCBA\n{:026b}\n",
            self.val.count_ones(),
            self.val
        )
    }
}

impl Edge {
    fn from_path(path: &[Point], grid: &[Vec<Tile>]) -> Edge {
        let doors = path
            .iter()
            .map(|(x, y)| grid[*y][*x])
            .filter_map(|x| match x {
                Tile::Door(d) => Some(Key::from(d.to_ascii_lowercase())),
                _ => None,
            });
        let mut keys_needed = Keys::default();
        doors.into_iter().for_each(|x| keys_needed.add_key(x));
        Edge {
            keys_needed,
            len: (path.len() - 1).try_into().unwrap(),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Door(char),
    Key(char),
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            '@' => Tile::Key('@'),
            c if c.is_ascii_lowercase() => Tile::Key(c),
            c if c.is_ascii_uppercase() => Tile::Door(c),
            other => panic!("Invalid character {other}"),
        }
    }
}

fn get_neigh(a: Point, grid: &[Vec<Tile>]) -> Vec<Point> {
    let mut neigh = Vec::with_capacity(4);
    if a.0 > 0 && grid[a.1][a.0 - 1] != Tile::Wall {
        neigh.push((a.0 - 1, a.1));
    }
    if a.0 < grid[0].len() && grid[a.1][a.0 + 1] != Tile::Wall {
        neigh.push((a.0 + 1, a.1));
    }
    if a.1 > 0 && grid[a.1 - 1][a.0] != Tile::Wall {
        neigh.push((a.0, a.1 - 1));
    }
    if a.1 < grid.len() && grid[a.1 + 1][a.0] != Tile::Wall {
        neigh.push((a.0, a.1 + 1));
    }

    neigh
}

fn manhattan(a: Point, b: Point) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn best_path(start: Point, end: Point, grid: &[Vec<Tile>]) -> Edge {
    assert!(start != end);
    let mut came_from = HashMap::new();
    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(start, 0);

    let mut q = PriorityQueue::new();
    q.push(start, Reverse(0));

    while let Some((p, _)) = q.pop() {
        if p == end {
            let mut path = Vec::with_capacity(cost_so_far[&end]);
            path.push(end);
            let mut pt = end;
            while let Some(parent) = came_from.get(&pt) {
                path.push(*parent);
                pt = *parent;
            }
            return Edge::from_path(&path, grid);
        }

        for i in get_neigh(p, grid) {
            let new_cost = cost_so_far[&p] + manhattan(i, end);
            if new_cost < *cost_so_far.get(&i).unwrap_or(&usize::MAX) {
                cost_so_far.insert(i, new_cost);
                let priority = new_cost + manhattan(i, end);
                q.push(i, Reverse(priority));
                came_from.insert(i, p);
            }
        }
    }
    panic!("Can't reach {end:?} from {start:?}");
}

type Quadrant = u8;
fn quadrant(x: Point) -> Quadrant {
    // start nodes are their own thing
    if x.0 < 40 {
        if x.1 < 40 {
            1
        } else {
            2
        }
    } else if x.1 < 40 {
        3
    } else {
        4
    }
}
const KEY_NUM: u32 = 26;
pub fn generate(input: &str, part2: bool) -> HashMap<Key, Vec<(Key, Edge, Quadrant)>> {
    let mut input = input.to_owned();
    let line_len = input.lines().next().unwrap().len() + 1;
    if part2 {
        input.replace_range(39 * line_len + 39..39 * line_len + 42, "@#@");
        input.replace_range(40 * line_len + 39..40 * line_len + 42, "###");
        input.replace_range(41 * line_len + 39..41 * line_len + 42, "@#@");
    }
    let key_positions: HashMap<(usize, usize), char> =
        HashMap::from_iter(input.lines().enumerate().flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| c.is_ascii_lowercase())
                .map(move |(x, c)| ((x, y), c))
        }));

    let start_pos: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '@')
                .map(move |(x, _)| (x, y))
        })
        .collect();

    let mut output: HashMap<Key, Vec<(Key, Edge, Quadrant)>> = HashMap::new();
    // makes indexing into the data easier
    let input: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| l.chars().map(Tile::from).collect())
        .collect();

    for (x, y) in key_positions.iter().tuple_combinations() {
        let quadr = quadrant(*x.0);
        if !part2 || quadr == quadrant(*y.0) {
            let (xk, yk) = ((*x.1).into(), (*y.1).into());
            let p = best_path(*x.0, *y.0, &input);
            output.entry(xk).or_default().push((yk, p, quadr));
            output.entry(yk).or_default().push((xk, p, quadr));
        }
    }
    for s in start_pos {
        let squadr = quadrant(s);
        for x in key_positions
            .iter()
            .filter(|x| !part2 || quadrant(*x.0) == squadr)
        {
            let xk = (*x.1).into();
            let p = best_path(s, *x.0, &input);
            //output.entry(xk).or_default().push((START, p));
            output.entry(START).or_default().push((xk, p, squadr));
        }
    }
    output
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct State {
    cur: Key,
    keys_owned: Keys,
    len: u32,
}

impl State {
    fn can_cross(&self, e: Edge) -> bool {
        self.keys_owned.can_cross(e)
    }
    fn to_hmap(self) -> (Key, Keys) {
        (self.cur, self.keys_owned)
    }
}

fn solve(graph: &HashMap<Key, Vec<(Key, Edge, Quadrant)>>, keys_available: Keys) -> u32 {
    let mut q: VecDeque<State> = graph
        .get(&START)
        .expect("No connected paths to start")
        .iter()
        .filter(|x| keys_available.can_cross(x.1))
        .copied()
        .map(|(k, e, _)| State {
            cur: k,
            keys_owned: {
                let mut a = keys_available;
                a.add_key(k);
                a
            },
            len: e.len,
        })
        .collect();
    let mut seen = HashMap::new();
    let mut best = u32::MAX;
    while let Some(s) = q.pop_front() {
        if let Some(old) = seen.get(&s.to_hmap()) {
            if s.len > *old {
                continue;
            }
        }
        seen.insert(s.to_hmap(), s.len);
        if s.keys_owned.val.count_ones() == KEY_NUM {
            best = std::cmp::min(best, s.len);
            continue;
        }
        for n in &graph[&s.cur] {
            if !s.keys_owned.has_key(n.0) && s.can_cross(n.1) {
                let mut new_state = s;
                new_state.keys_owned.add_key(n.0);
                new_state.len += n.1.len;
                new_state.cur = n.0;
                let old = seen.entry(new_state.to_hmap()).or_insert(u32::MAX);

                if new_state.len < *old {
                    *old = new_state.len;
                    q.push_back(new_state);
                }
            }
        }
    }

    best
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> u32 {
    let graph = generate(input, false);

    solve(&graph, Keys::default())
}

// Just assume you already have all the other keys when solving a quadrant
#[aoc(day18, part2)]
pub fn part2(input: &str) -> u32 {
    let graph = generate(input, true);
    let start = graph[&START].clone();

    let (mut q1, rest): (HashMap<_, _>, HashMap<_, _>) = graph
        .into_iter()
        .partition(|x| x.1[0].2 == 1 && x.0 != START);
    let (mut q2, rest): (HashMap<_, _>, HashMap<_, _>) = rest
        .into_iter()
        .partition(|x| x.1[0].2 == 2 && x.0 != START);
    let (mut q3, mut q4): (HashMap<_, _>, HashMap<_, _>) = rest
        .into_iter()
        .partition(|x| x.1[0].2 == 3 && x.0 != START);

    for q in [&mut q1, &mut q2, &mut q3, &mut q4] {
        q.insert(START, start.clone());
    }

    let mut keys_avail1 = Keys::default();
    let mut keys_avail2 = Keys::default();
    let mut keys_avail3 = Keys::default();
    let mut keys_avail4 = Keys::default();

    q1.keys().for_each(|k| {
        keys_avail2.add_key(*k);
        keys_avail3.add_key(*k);
        keys_avail4.add_key(*k);
    });
    q2.keys().for_each(|k| {
        keys_avail1.add_key(*k);
        keys_avail3.add_key(*k);
        keys_avail4.add_key(*k);
    });
    q3.keys().for_each(|k| {
        keys_avail1.add_key(*k);
        keys_avail2.add_key(*k);
        keys_avail4.add_key(*k);
    });
    q4.keys().for_each(|k| {
        keys_avail1.add_key(*k);
        keys_avail2.add_key(*k);
        keys_avail3.add_key(*k);
    });

    q1.entry(START)
        .and_modify(|x| x.retain(|(k, _, _)| !keys_avail1.has_key(*k)));
    q2.entry(START)
        .and_modify(|x| x.retain(|(k, _, _)| !keys_avail2.has_key(*k)));
    q3.entry(START)
        .and_modify(|x| x.retain(|(k, _, _)| !keys_avail3.has_key(*k)));
    q4.entry(START)
        .and_modify(|x| x.retain(|(k, _, _)| !keys_avail4.has_key(*k)));

    solve(&q1, keys_avail1)
        + solve(&q2, keys_avail2)
        + solve(&q3, keys_avail3)
        + solve(&q4, keys_avail4)
}

// steps to solve thing
//
// 1. parse maze into undirected graph
//    (edges between keys also store which doors you need to pass through)
//    (multiple paths between two keys may be possible)
// 2. find all keys that you can reach with no keys held (have to start with them)
// 3. mark that key as collected, then find all new reachable keys when you have that key.
// 4. you know the total path when you collect the last key.
//
// store the keys held/required for edge as u32 because its fast
//
// can pass: if req == req & held
// add key to held: held |= 1 << (0 for a, 25 for z)
// has key: held & 1 << (0..=25) != 0
//
// the graph is not directed
//
// if state (keys obtained, at position) -> steps already exists with smaller number of steps, just
// don't explore it, otherwise keep exploring
//
// for going from one key to another, choose the path that is the shortest that has no doors you
// can't unlock.
//
// best path key1 -> key2 is the same path as key2 -> key1, so hashmap of edges shouldn't matter on
// the order of elements.
// function ord(one: Key, two: Key) -> (Key, Key)
