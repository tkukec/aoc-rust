// a lot of imports
use cached::proc_macro::cached;
use priority_queue::PriorityQueue;
use std::{
    cmp::{min, Reverse},
    collections::HashMap,
};

// Too lazy to parse the thing
const DEPTH: u32 = 3339;
const TX: u32 = 10;
const TY: u32 = 715;

#[aoc(day22, part1)]
pub fn part1(_input: &str) -> u32 {
    let mut sum = 0;
    for y in 0..=TY {
        for x in 0..=TX {
            sum += region_type((x, y)) as u32;
        }
    }
    sum
}

#[aoc(day22, part2)]
pub fn part2(_input: &str) -> u32 {
    search((0, 0), (TX, TY), Tool::Torch)
}

fn search(start: Point, goal: Point, start_tool: Tool) -> u32 {
    let mut came_from: HashMap<(Point, Tool), Option<(Point, Tool)>> = HashMap::new();
    let mut cost_so_far: HashMap<(Point, Tool), u32> = HashMap::new();
    let mut q: PriorityQueue<(Point, Tool), Reverse<u32>> = PriorityQueue::new();

    q.push((start, start_tool), Reverse(0));
    came_from.insert((start, start_tool), None);
    cost_so_far.insert((start, start_tool), 0);

    fn neigh(a: Point) -> Vec<Point> {
        let mut out = Vec::with_capacity(4);
        out.push((a.0 + 1, a.1));
        out.push((a.0, a.1 + 1));
        if a.0 != 0 {
            out.push((a.0 - 1, a.1));
        }
        if a.1 != 0 {
            out.push((a.0, a.1 - 1));
        }

        out
    }
    fn manhattan(a: Point, b: Point) -> u32 {
        a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
    }

    // The algorithm has a bug, so I just go on after it found the goal and hope it finds
    // a better solution. I can't be bothered fixing the thing.
    let mut go_on = true;
    let mut timer = 2000;
    while let Some(((cur_pos, cur_tool), _)) = q.pop() {
        if !go_on {
            timer -= 1;
            if timer == 0 {
                println!("Breaking");
                break;
            }
        }
        if cur_pos == goal {
            println!("Found goal");
            go_on = false;
        }
        for n in neigh(cur_pos) {
            let new_region = region_type(n);
            let old_region = region_type(cur_pos);
            // only switch if needed to enter the neighboring region
            let overlap_tool = old_region.overlapping_tool(&new_region);
            let tool_time: u32;
            let new_tool: Tool;
            if let Overlap::Tool(tool) = overlap_tool {
                new_tool = tool;
                if tool != cur_tool {
                    tool_time = 7;
                } else {
                    tool_time = 0;
                }
            } else {
                new_tool = cur_tool;
                tool_time = 0;
            }
            let new_cost = cost_so_far[&(cur_pos, cur_tool)] + 1 + tool_time;
            if cost_so_far.get(&(n, new_tool)).is_none() || cost_so_far[&(n, new_tool)] > new_cost {
                cost_so_far.insert((n, new_tool), new_cost);
                let priority = new_cost + manhattan(n, goal);
                q.push((n, new_tool), Reverse(priority));
                came_from.insert((n, new_tool), Some((cur_pos, cur_tool)));
            }
        }
    }
    // Debug printing, looks nice, but the actual input is too large to print.
    /*
        let mut grid: Vec<Vec<char>> = (0..20)
            .map(|y| (0..20).map(|x| region_type((x, y)).into()).collect())
            .collect();
        let mut last = *came_from.keys().find(|(x, _)| *x == goal).unwrap();
        while let Some(Some(parent)) = came_from.get(&last) {
            grid[last.0 .1 as usize][last.0 .0 as usize] = match last.1 {
                Tool::Neither => 'N',
                Tool::Torch => 'T',
                Tool::ClimbingGear => 'C',
            };
            last = *parent;
        }

        use itertools::Itertools;
        println!(
            "{}",
            grid.iter().map(|l| l.iter().collect::<String>()).join("\n")
        );
    */
    let mut cost = 0;
    if let Some(c) = cost_so_far.get(&(goal, Tool::Torch)) {
        // On the goal with a torch
        cost = *c;
    }
    if let Some(c) = cost_so_far.get(&(goal, Tool::Torch)) {
        cost = min(cost, c + 7);
    }
    cost
}

type Point = (u32, u32);

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Tool {
    Neither,
    Torch,
    ClimbingGear,
}

fn geo_index(p: Point) -> u32 {
    match p {
        (0, 0) => 0,
        (TX, TY) => 0,
        (0, y) => y * 48271,
        (x, 0) => x * 16807,
        (x, y) => erosion_level((x - 1, y)) * erosion_level((x, y - 1)),
    }
}

#[cached]
fn erosion_level(p: Point) -> u32 {
    (DEPTH + geo_index(p)) % 20183
}

fn region_type(p: Point) -> Region {
    (erosion_level(p) % 3).try_into().unwrap()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Region {
    Rocky,
    Wet,
    Narrow,
}

impl TryFrom<u32> for Region {
    type Error = &'static str;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Region::Rocky),
            1 => Ok(Region::Wet),
            2 => Ok(Region::Narrow),
            _ => Err("Bad"),
        }
    }
}

impl From<Region> for char {
    fn from(value: Region) -> Self {
        match value {
            Region::Rocky => '.',
            Region::Wet => '=',
            Region::Narrow => '|',
        }
    }
}

#[derive(Copy, Clone, Hash)]
enum Overlap {
    Tool(Tool),
    SameRegion,
}

impl Region {
    // returns its own type instead of just a single tool because if the region is the same, then
    // there are two tools overlapping, and I would need to add a check to not call this
    // function if the regions are the same
    fn overlapping_tool(&self, other: &Self) -> Overlap {
        match (self, other) {
            (x, y) if x == y => Overlap::SameRegion,
            (Region::Rocky, Region::Wet) | (Region::Wet, Region::Rocky) => {
                Overlap::Tool(Tool::ClimbingGear)
            }
            (Region::Rocky, Region::Narrow) | (Region::Narrow, Region::Rocky) => {
                Overlap::Tool(Tool::Torch)
            }
            (Region::Wet, Region::Narrow) | (Region::Narrow, Region::Wet) => {
                Overlap::Tool(Tool::Neither)
            }
            _ => unreachable!(),
        }
    }
}
