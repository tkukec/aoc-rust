mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

aoc_lib! {year=2022}

pub mod generators {
    use std::{
        num::ParseIntError,
        str::{FromStr, Lines},
    };

    use itertools::Itertools;
    pub fn num_lines<T>(input: &str) -> Vec<T>
    where
        T: FromStr<Err = ParseIntError>,
    {
        input.lines().map(|x| x.parse().unwrap()).collect()
    }

    pub fn num_split_by<T>(input: &str, sep: &str) -> Vec<T>
    where
        T: FromStr<Err = ParseIntError>,
    {
        input.split(sep).map(|x| x.parse().unwrap()).collect()
    }

    pub fn to_grid<T>(input: &str) -> Vec<Vec<T>>
    where
        T: From<char>,
    {
        input
            .lines()
            .map(|x| x.chars().map(|c| T::from(c)).collect())
            .collect()
    }

    pub fn bin_lines(input: &str) -> Vec<u32> {
        input
            .lines()
            .map(|x| u32::from_str_radix(x, 2).unwrap())
            .collect()
    }

    pub trait GroupThings<T>: Iterator<Item = T> {
        fn split(self, sep: T) -> Vec<Vec<T>>
        where
            Self: Sized,
            T: PartialEq,
        {
            self.group_by(|x| *x == sep)
                .into_iter()
                .filter(|(k, _)| !k)
                .map(|(_, x)| x.collect())
                .collect()
        }
    }

    impl<T> GroupThings<T> for dyn Iterator<Item = T> {}
    impl<T> GroupThings<T> for std::vec::IntoIter<T> {}

    impl<'a> GroupThings<&'a str> for Lines<'a> {}
}

pub mod spatial {
    use std::{
        fmt::Debug,
        marker::Sized,
        ops::{Index, IndexMut},
    };

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    impl Point {
        pub fn manhattan(self, other: Self) -> u32 {
            self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
        }

        pub fn from(a: i32, b: i32) -> Self {
            Point { x: a, y: b }
        }

        pub fn apply_velocity(&mut self, vel: Point) {
            self.x += vel.x;
            self.y += vel.y;
        }
    }

    impl From<(i32, i32)> for Point {
        fn from(a: (i32, i32)) -> Self {
            Point { x: a.0, y: a.1 }
        }
    }

    impl From<&str> for Point {
        fn from(a: &str) -> Self {
            let a = a.split_once(',').unwrap();
            let a = (a.0.parse().unwrap(), a.1.parse().unwrap());
            Point { x: a.0, y: a.1 }
        }
    }

    impl Debug for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Point({}, {})", self.x, self.y)
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
    pub struct Point3D {
        pub x: i32,
        pub y: i32,
        pub z: i32,
    }

    impl Point3D {
        pub fn manhattan(self, other: Self) -> u32 {
            self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
        }

        pub fn from(x: i32, y: i32, z: i32) -> Self {
            Point3D { x, y, z }
        }

        pub fn apply_velocity(&mut self, vel: Point3D) {
            self.x += vel.x;
            self.y += vel.y;
            self.z += vel.z;
        }
    }

    impl From<(i32, i32, i32)> for Point3D {
        fn from(a: (i32, i32, i32)) -> Self {
            Point3D {
                x: a.0,
                y: a.1,
                z: a.2,
            }
        }
    }

    impl From<&str> for Point3D {
        fn from(a: &str) -> Self {
            let mut a = a.split(',');
            let a = (
                a.next().unwrap().parse().unwrap(),
                a.next().unwrap().parse().unwrap(),
                a.next().unwrap().parse().unwrap(),
            );
            Point3D {
                x: a.0,
                y: a.1,
                z: a.2,
            }
        }
    }

    impl Debug for Point3D {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Point3D({}, {}, {})", self.x, self.y, self.z)
        }
    }

    // ugly, but works. just accepted compiler suggestions until it worked
    pub trait GetFromPoint: Index<usize> + Sized {
        fn get_pt(&self, pt: Point) -> &<<Self as Index<usize>>::Output as Index<usize>>::Output
        where
            <Self as Index<usize>>::Output: Index<usize>,
            <<Self as Index<usize>>::Output as Index<usize>>::Output: Sized,
        {
            assert!(pt.x >= 0 && pt.y >= 0);
            &self[pt.y as usize][pt.x as usize]
        }
    }

    // arguably even worse than the previous one
    // why does Index have Index<_>::Output, but IndexMut doesn't?
    pub trait GetFromPointMut: Index<usize> + Sized + IndexMut<usize> {
        fn get_pt_mut(
            &mut self,
            pt: Point,
        ) -> &mut <<Self as Index<usize>>::Output as Index<usize>>::Output
        where
            <Self as Index<usize>>::Output: IndexMut<usize>,
            <<Self as Index<usize>>::Output as Index<usize>>::Output: Sized,
        {
            assert!(pt.x >= 0 && pt.y >= 0);
            self.index_mut(pt.y as usize).index_mut(pt.x as usize)
        }
    }

    impl<T> GetFromPoint for Vec<Vec<T>> {}
    impl<T> GetFromPointMut for Vec<Vec<T>> {}
}

pub mod pathfinding {
    use std::collections::{HashMap, VecDeque};

    use crate::spatial::Point;

    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Results {
        pub cost: u32,
        pub path: Vec<Point>,
    }

    type GetNeighFn<T> = fn(Point, &[Vec<T>]) -> Vec<(Point, u32)>;

    // optimizing the utils for solving just the happy path,
    // so this isn't an Option or a Result, and just panics instead
    pub fn min_path_bfs<T>(
        start: Point,
        goal: Point,
        grid: &[Vec<T>],
        neigh_fn: GetNeighFn<T>,
    ) -> Results {
        let mut seen = HashMap::new();
        let mut came_from = HashMap::new();
        let mut q = VecDeque::new();

        q.push_back((start, 0));

        while let Some((pt, cost_to)) = q.pop_front() {
            if pt == goal {
                let mut path = Vec::new();
                let mut cur = goal;
                while let Some(parent) = came_from.get(&cur) {
                    path.push(*parent);
                    cur = *parent;
                }
                return Results {
                    cost: cost_to,
                    path,
                };
            }

            for (x, cost) in neigh_fn(pt, grid) {
                let new_cost = cost + cost_to;
                let old = seen.entry(x).or_insert(0);
                if *old > new_cost {
                    *old = new_cost;
                    q.push_back((x, new_cost));
                    came_from.insert(x, pt);
                }
            }
        }
        panic!("No path from start {:?} to goal {:?}", start, goal);
    }
}
