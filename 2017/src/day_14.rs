use std::collections::HashSet;

use itertools::Itertools;

#[aoc(day14, part1)]
pub fn part1(input: &str) -> u32 {
    (0..128)
        .map(|x| {
            let mut a: Vec<u8> = (0..=255).collect();
            let mut cur = 0;
            let a_len = a.len();
            for (skip_size, l) in std::iter::repeat(
                format!("{input}-{x}")
                    .bytes()
                    .map(|x| x as usize)
                    .chain([17, 31, 73, 47, 23]),
            )
            .take(64)
            .flatten()
            .enumerate()
            {
                if cur + l < a_len {
                    a[cur..(cur + l)].reverse();
                } else {
                    let mut first = a[cur..a_len].to_vec();
                    let mut second = a[0..((cur + l) % a_len)].to_vec();
                    first.append(&mut second);
                    first.reverse();
                    let mut it = first.into_iter();
                    a.get_mut(cur..a_len)
                        .unwrap()
                        .iter_mut()
                        .for_each(|x| *x = it.next().unwrap());
                    a.get_mut(0..((cur + l) % a_len))
                        .unwrap()
                        .iter_mut()
                        .for_each(|x| *x = it.next().unwrap());
                }
                cur += l + skip_size;
                cur %= a_len;
            }
            a.chunks(16)
                .map(|x| {
                    x.iter()
                        .copied()
                        .reduce(|accum, item| accum ^ item)
                        .unwrap()
                })
                .map(|x| x.count_ones())
                .sum::<u32>()
        })
        .sum()
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> i32 {
    let a = (0..128)
        .map(|x| {
            let mut a: Vec<u8> = (0..=255).collect();
            let mut cur = 0;
            let a_len = a.len();
            for (skip_size, l) in std::iter::repeat(
                format!("{input}-{x}")
                    .bytes()
                    .map(|x| x as usize)
                    .chain([17, 31, 73, 47, 23]),
            )
            .take(64)
            .flatten()
            .enumerate()
            {
                if cur + l < a_len {
                    a[cur..(cur + l)].reverse();
                } else {
                    let mut first = a[cur..a_len].to_vec();
                    let mut second = a[0..((cur + l) % a_len)].to_vec();
                    first.append(&mut second);
                    first.reverse();
                    let mut it = first.into_iter();
                    a.get_mut(cur..a_len)
                        .unwrap()
                        .iter_mut()
                        .for_each(|x| *x = it.next().unwrap());
                    a.get_mut(0..((cur + l) % a_len))
                        .unwrap()
                        .iter_mut()
                        .for_each(|x| *x = it.next().unwrap());
                }
                cur += l + skip_size;
                cur %= a_len;
            }
            a.chunks(16)
                .map(|x| {
                    x.iter()
                        .copied()
                        .reduce(|accum, item| accum ^ item)
                        .unwrap()
                })
                .map(|x| format!("{:08b}", x))
                .join("")
                .chars()
                .map(|x| x == '1')
                .collect()
        })
        .collect::<Vec<Vec<bool>>>();
    let mut points = HashSet::new();
    for (i, line) in a.iter().enumerate() {
        for (j, pt) in line.iter().enumerate() {
            if *pt {
                points.insert((j as u8, i as u8));
            }
        }
    }
    let mut regions = 0;

    while let Some(p) = points.iter().next().copied() {
        points.remove(&p);
        visit(p, &mut points);
        regions += 1;
    }

    regions
}

fn visit(i: (u8, u8), points: &mut HashSet<(u8, u8)>) {
    if points.remove(&(i.0 + 1, i.1)) {
        visit((i.0 + 1, i.1), points);
    }
    if points.remove(&(i.0 - 1, i.1)) {
        visit((i.0 - 1, i.1), points);
    }
    if points.remove(&(i.0, i.1 + 1)) {
        visit((i.0, i.1 + 1), points);
    }
    if points.remove(&(i.0, i.1 - 1)) {
        visit((i.0, i.1 - 1), points);
    }
}
