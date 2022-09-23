use std::collections::HashMap;

// https://stackoverflow.com/a/38169182
use std::convert::AsMut;
fn clone_into_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Clone,
{
    assert_eq!(
        slice.len(),
        std::mem::size_of::<A>() / std::mem::size_of::<T>()
    );

    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}

#[aoc_generator(day12)]
fn generate(input: &str) -> (Vec<bool>, HashMap<[bool; 5], bool>) {
    let initial = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .skip(15)
        .map(|x| x == '#')
        .collect();
    let rules = input
        .lines()
        .skip(2)
        .map(|l| {
            let (a, b) = l.split_once(" => ").unwrap();
            let before: [bool; 5] =
                clone_into_array(&a.chars().map(|x| x == '#').collect::<Vec<bool>>()[0..5]);
            let after = b.chars().map(|x| x == '#').next().unwrap();
            (before, after)
        })
        .collect();
    (initial, rules)
}

#[aoc(day12, part1)]
pub fn part1(input: &(Vec<bool>, HashMap<[bool; 5], bool>)) -> i32 {
    let mut initial = vec![false; 10]; // the flowers don't really spread to the left a lot
    let (mut i, data) = input.clone();
    initial.append(&mut i);
    initial.append(&mut vec![false; 50]); // additional space for the flowers to spread
    for _ in 0..20 {
        initial = initial
            .array_windows()
            .map(|x| *data.get(x).unwrap_or(&false))
            .collect();
        initial.insert(0, false);
        initial.insert(0, false);
        initial.append(&mut vec![false; 2]);
    }
    initial
        .iter()
        .enumerate()
        .filter(|x| *x.1)
        .map(|x| x.0 as i32 - 10)
        .sum()
}

#[aoc(day12, part2)]
pub fn part2(input: &(Vec<bool>, HashMap<[bool; 5], bool>)) -> usize {
    let mut initial = vec![false; 10];
    let (mut i, data) = input.clone();
    initial.append(&mut i);
    initial.append(&mut vec![false; 110]); // give the flowers enough space to spread
    for _ in 0..100 {
        initial = initial
            .array_windows()
            .map(|x| *data.get(x).unwrap_or(&false))
            .collect();
        initial.insert(0, false);
        initial.insert(0, false);
        initial.append(&mut vec![false; 2]);
    }
    // at n = 97, the flowers stabilize into an unchanging shape that just keeps moving to the
    // right by one square each iteration. I do a 100 iterations and then just add the offset
    initial
        .iter()
        .enumerate()
        .filter(|x| *x.1)
        .map(|x| x.0 as usize - 110 + 50000000000)
        .sum()
}
