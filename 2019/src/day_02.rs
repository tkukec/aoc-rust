#[aoc(day02, part1)]
pub fn part1(input: &str) -> u32 {
    let mut a: Vec<u32> = input
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut ptr = 0;
    a[1] = 12;
    a[2] = 2;
    loop {
        match a[ptr] {
            1 => {
                let v: u32 = a[a[ptr + 1] as usize] + a[a[ptr + 2] as usize];
                let index = a[ptr + 3] as usize; // borrow rules
                a[index] = v;
            }
            2 => {
                let v: u32 = a[a[ptr + 1] as usize] * a[a[ptr + 2] as usize];
                let index = a[ptr + 3] as usize; // borrow rules
                a[index] = v;
            }
            99 => {
                return a[0];
            }
            _ => panic!("Invalid opcode at {ptr}: {}", a[ptr]),
        }
        ptr += 4;
    }
}

#[aoc(day02, part2)]
pub fn part2(input: &str) -> u32 {
    let a: Vec<u32> = input
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    const GOAL: u32 = 19690720;
    for noun in 0..=99 {
        'outer: for verb in 0..=99 {
            let mut a = a.clone();

            let mut ptr = 0;
            a[1] = noun;
            a[2] = verb;
            loop {
                match a[ptr] {
                    1 => {
                        let v: u32 = a[a[ptr + 1] as usize] + a[a[ptr + 2] as usize];
                        let index = a[ptr + 3] as usize; // borrow rules
                        a[index] = v;
                    }
                    2 => {
                        let v: u32 = a[a[ptr + 1] as usize] * a[a[ptr + 2] as usize];
                        let index = a[ptr + 3] as usize; // borrow rules
                        a[index] = v;
                    }
                    99 => {
                        if a[0] == GOAL {
                            return 100 * noun + verb;
                        } else {
                            continue 'outer;
                        }
                    }
                    _ => panic!("Invalid opcode at {ptr}: {}", a[ptr]),
                }
                ptr += 4;
            }
        }
    }
    panic!("No good input");
}
