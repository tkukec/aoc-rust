#[aoc(day01, part1)]
pub fn part1(input: &str) -> i32 {
    let mut cur_dir = 'N';
    let mut x = 0;
    let mut y = 0;
    for (d, num) in input.split(", ").map(|x| x.split_at(1)) {
        let num: i32 = num.parse().unwrap();
        cur_dir = match d {
            "L" => match cur_dir {
                'N' => 'W',
                'W' => 'S',
                'S' => 'E',
                'E' => 'N',
                _ => unreachable!(),
            },
            "R" => match cur_dir {
                'N' => 'E',
                'E' => 'S',
                'S' => 'W',
                'W' => 'N',
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        match cur_dir {
            'N' => x += num,
            'E' => y += num,
            'S' => x -= num,
            'W' => y -= num,
            _ => unreachable!(),
        }
    }
    x.abs() + y.abs()
}

#[aoc(day01, part2)]
pub fn part2(input: &str) -> i32 {
    let mut cur_dir = 'N';
    let mut x = 0;
    let mut y = 0;
    let mut visited = vec![];
    for (d, num) in input.split(", ").map(|x| x.split_at(1)) {
        let num: i32 = num.parse().unwrap();

        cur_dir = match d {
            "L" => match cur_dir {
                'N' => 'W',
                'W' => 'S',
                'S' => 'E',
                'E' => 'N',
                _ => unreachable!(),
            },
            "R" => match cur_dir {
                'N' => 'E',
                'E' => 'S',
                'S' => 'W',
                'W' => 'N',
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        match cur_dir {
            'N' => y += num,
            'E' => x += num,
            'S' => y -= num,
            'W' => x -= num,
            _ => unreachable!(),
        }
        match cur_dir {
            'N' => ((y - num + 1)..=y).for_each(|y1| visited.push((x, y1))),
            'E' => ((x - num + 1)..=x).for_each(|x1| visited.push((x1, y))),
            'S' => (y..(y + num)).for_each(|y1| visited.push((x, y1))),
            'W' => (x..(x + num)).for_each(|x1| visited.push((x1, y))),
            _ => unreachable!(),
        }
        visited.sort();
        if let Some(res) = visited.windows(2).find(|x| x[0] == x[1]) {
            return res[0].0.abs() + res[0].1.abs();
        }
    }
    -1
}
