use itertools::Itertools;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> i32 {
    let mut a: Vec<u8> = (0..=255).collect();
    let mut cur = 0;
    let a_len = a.len();
    for (skip_size, l) in input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
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

    a[0] as i32 * a[1] as i32
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> String {
    let mut a: Vec<u8> = (0..=255).collect();
    let mut cur = 0;
    let a_len = a.len();
    for (skip_size, l) in std::iter::repeat(
        input
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
        .map(|x| x.iter().copied().reduce(|accum, item| accum ^ item))
        .map(|x| format!("{:02x}", x.unwrap()))
        .join("")
}
