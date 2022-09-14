use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> u32 {
    let mut blocks: Vec<u32> = input.split('\t').map(|x| x.parse().unwrap()).collect();
    //let mut blocks = vec![0, 2, 7, 0];
    let mut seen = HashSet::new();
    let len = blocks.len();
    let mut cnt = 0;
    loop {
        if seen.contains(&blocks) {
            break;
        }
        cnt += 1;
        seen.insert(blocks.clone());
        let (max_ind, max_val) = blocks
            .iter()
            .copied()
            .enumerate()
            .max_by(|(i1, v1), (i2, v2)| v1.cmp(v2).then(i1.cmp(i2).reverse()))
            .unwrap();
        blocks[max_ind] = 0;
        let additional_added = max_val as usize % len; // how many banks get one more block
        for (i, j) in blocks.iter_mut().enumerate() {
            *j += max_val / len as u32
                + ((i > max_ind && i <= (max_ind + additional_added))
                    || (max_ind + additional_added >= len
                        && i <= (max_ind + additional_added) % len)) as u32
        }
    }
    cnt
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let mut blocks: Vec<u32> = input.split('\t').map(|x| x.parse().unwrap()).collect();
    //let mut blocks = vec![0, 2, 7, 0];
    let mut seen = vec![];
    let len = blocks.len();
    loop {
        if seen.contains(&blocks) {
            return seen.len() - seen.iter().position(|r| r == &blocks).unwrap();
        }
        seen.push(blocks.clone());
        let (max_ind, max_val) = blocks
            .iter()
            .copied()
            .enumerate()
            .max_by(|(i1, v1), (i2, v2)| v1.cmp(v2).then(i1.cmp(i2).reverse()))
            .unwrap();
        blocks[max_ind] = 0;
        let additional_added = max_val as usize % len; // how many banks get one more block
        for (i, j) in blocks.iter_mut().enumerate() {
            *j += max_val / len as u32
                + ((i > max_ind && i <= (max_ind + additional_added))
                    || (max_ind + additional_added >= len
                        && i <= (max_ind + additional_added) % len)) as u32
        }
    }
}
