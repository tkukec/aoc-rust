use std::collections::HashMap;

// 1 2
// 3 4
//
// rot
// 3 1
// 4 2
//
// rot x2
// 4 3
// 2 1
//
// rot x3
// 2 4
// 1 3
//
// flip
// 2 1
// 4 3
//
// flip rot
// 4 2
// 3 1
//
// flip rot x2
// 3 4
// 1 2
//
// flip rot x3
// 1 3
// 2 4

#[aoc_generator(day21)]
pub fn generate(input: &str) -> HashMap<Vec<Vec<char>>, Vec<Vec<char>>> {
    let mut a = HashMap::new();

    for mut l in input.lines().map(|x| x.split(" => ")) {
        let first = l.next().unwrap();
        let second = l.next().unwrap();
        let mut first: Vec<Vec<char>> = first.split('/').map(|x| x.chars().collect()).collect();
        let second: Vec<Vec<char>> = second.split('/').map(|x| x.chars().collect()).collect();
        a.insert(first.clone(), second.clone());
        rotate(&mut first);
        a.insert(first.clone(), second.clone());
        rotate(&mut first);
        a.insert(first.clone(), second.clone());
        rotate(&mut first);
        a.insert(first.clone(), second.clone());
        rotate(&mut first);
        flip(&mut first);
        a.insert(first.clone(), second.clone());
        rotate(&mut first);
        a.insert(first.clone(), second.clone());
        rotate(&mut first);
        a.insert(first.clone(), second.clone());
        rotate(&mut first);
        a.insert(first.clone(), second.clone());
    }
    a
}

// rotate clockwise 90 deg
// https://www.geeksforgeeks.org/rotate-a-matrix-by-90-degree-in-clockwise-direction-without-using-any-extra-space/
fn rotate(grid: &mut [Vec<char>]) {
    let n = grid.len();
    for i in 0..(n / 2) {
        for j in i..(n - i - 1) {
            let temp = grid[i][j];
            grid[i][j] = grid[n - 1 - j][i];
            grid[n - 1 - j][i] = grid[n - 1 - i][n - 1 - j];
            grid[n - 1 - i][n - 1 - j] = grid[j][n - 1 - i];
            grid[j][n - 1 - i] = temp;
        }
    }
}

// flip L | R
fn flip(grid: &mut [Vec<char>]) {
    grid.iter_mut().for_each(|x| x.reverse())
}

#[aoc(day21, part1)]
pub fn part1(input: &HashMap<Vec<Vec<char>>, Vec<Vec<char>>>) -> usize {
    let mut grid = vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#'],
    ];

    rotate(&mut grid);
    rotate(&mut grid);
    rotate(&mut grid);
    for _ in 0..5 {
        let l = grid.len();
        let mut new_grid: Vec<Vec<char>> = std::iter::repeat(vec![])
            .take(if l % 2 == 0 { (l / 2) * 3 } else { (l / 3) * 4 })
            .collect();
        if l % 2 == 0 {
            for i in (0..l).step_by(2) {
                for j in (0..l).step_by(2) {
                    let chunk = vec![
                        grid[j][i..(i + 2)].to_vec(),
                        grid[j + 1][i..(i + 2)].to_vec(),
                    ];
                    let mut new_chunk = input.get(&chunk).unwrap().clone();
                    new_grid
                        .get_mut(j / 2 * 3)
                        .unwrap()
                        .append(&mut new_chunk[0]);
                    new_grid
                        .get_mut(j / 2 * 3 + 1)
                        .unwrap()
                        .append(&mut new_chunk[1]);
                    new_grid
                        .get_mut(j / 2 * 3 + 2)
                        .unwrap()
                        .append(&mut new_chunk[2]);
                }
            }
        } else {
            for i in (0..l).step_by(3) {
                for j in (0..l).step_by(3) {
                    let chunk = vec![
                        grid[j][i..(i + 3)].to_vec(),
                        grid[j + 1][i..(i + 3)].to_vec(),
                        grid[j + 2][i..(i + 3)].to_vec(),
                    ];
                    let mut new_chunk = input.get(&chunk).unwrap().clone();
                    new_grid
                        .get_mut(i / 3 * 4)
                        .unwrap()
                        .append(&mut new_chunk[0]);
                    new_grid
                        .get_mut(i / 3 * 4 + 1)
                        .unwrap()
                        .append(&mut new_chunk[1]);
                    new_grid
                        .get_mut(i / 3 * 4 + 2)
                        .unwrap()
                        .append(&mut new_chunk[2]);
                    new_grid
                        .get_mut(i / 3 * 4 + 3)
                        .unwrap()
                        .append(&mut new_chunk[3]);
                }
            }
        }
        grid = new_grid;
    }

    grid.iter().flatten().filter(|x| **x == '#').count()
}

#[aoc(day21, part2)]
pub fn part2(input: &HashMap<Vec<Vec<char>>, Vec<Vec<char>>>) -> usize {
    let mut grid = vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#'],
    ];

    rotate(&mut grid);
    rotate(&mut grid);
    rotate(&mut grid);
    for _ in 0..18 {
        let l = grid.len();
        let mut new_grid: Vec<Vec<char>> = std::iter::repeat(vec![])
            .take(if l % 2 == 0 { (l / 2) * 3 } else { (l / 3) * 4 })
            .collect();
        if l % 2 == 0 {
            for i in (0..l).step_by(2) {
                for j in (0..l).step_by(2) {
                    let chunk = vec![
                        grid[j][i..(i + 2)].to_vec(),
                        grid[j + 1][i..(i + 2)].to_vec(),
                    ];
                    let mut new_chunk = input.get(&chunk).unwrap().clone();
                    new_grid
                        .get_mut(j / 2 * 3)
                        .unwrap()
                        .append(&mut new_chunk[0]);
                    new_grid
                        .get_mut(j / 2 * 3 + 1)
                        .unwrap()
                        .append(&mut new_chunk[1]);
                    new_grid
                        .get_mut(j / 2 * 3 + 2)
                        .unwrap()
                        .append(&mut new_chunk[2]);
                }
            }
        } else {
            for i in (0..l).step_by(3) {
                for j in (0..l).step_by(3) {
                    let chunk = vec![
                        grid[j][i..(i + 3)].to_vec(),
                        grid[j + 1][i..(i + 3)].to_vec(),
                        grid[j + 2][i..(i + 3)].to_vec(),
                    ];
                    let mut new_chunk = input.get(&chunk).unwrap().clone();
                    new_grid
                        .get_mut(i / 3 * 4)
                        .unwrap()
                        .append(&mut new_chunk[0]);
                    new_grid
                        .get_mut(i / 3 * 4 + 1)
                        .unwrap()
                        .append(&mut new_chunk[1]);
                    new_grid
                        .get_mut(i / 3 * 4 + 2)
                        .unwrap()
                        .append(&mut new_chunk[2]);
                    new_grid
                        .get_mut(i / 3 * 4 + 3)
                        .unwrap()
                        .append(&mut new_chunk[3]);
                }
            }
        }
        grid = new_grid;
    }

    grid.iter().flatten().filter(|x| **x == '#').count()
}
