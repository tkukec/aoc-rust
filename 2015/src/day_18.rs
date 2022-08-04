const GRID_SIZE: i32 = 100;
fn generate(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|x| x.chars().map(|x| x == '#').collect())
        .collect()
}
fn neighbours(line_num: i32, char_num: i32, grid: &[Vec<bool>]) -> i32 {
    let neigh = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    neigh
        .into_iter()
        .map(|(x, y)| {
            let new_line = line_num + x;
            let new_char = char_num + y;
            (0..GRID_SIZE).contains(&new_line)
                && (0..GRID_SIZE).contains(&new_char)
                && grid[new_line as usize][new_char as usize]
        })
        .filter(|x| *x)
        .count() as i32
}
fn iterate(old_grid: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_grid = vec![];
    for line_num in 0..GRID_SIZE {
        let mut new_line = vec![];
        for char_num in 0..GRID_SIZE {
            new_line.push(if old_grid[line_num as usize][char_num as usize] {
                let num = neighbours(line_num, char_num, &old_grid);
                num == 2 || num == 3
            } else {
                neighbours(line_num, char_num, &old_grid) == 3
            });
        }
        new_grid.push(new_line);
    }
    new_grid
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> i32 {
    let mut grid: Vec<Vec<bool>> = generate(input);
    for _ in 0..100 {
        grid = iterate(grid);
    }
    grid.into_iter().flatten().filter(|x| *x).count() as i32
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> i32 {
    let mut grid: Vec<Vec<bool>> = generate(input);
    let max = (GRID_SIZE - 1) as usize;
    grid[0][0] = true;
    grid[0][max] = true;
    grid[max][0] = true;
    grid[max][max] = true;
    for _ in 0..100 {
        grid = iterate(grid);
        grid[0][0] = true;
        grid[0][max] = true;
        grid[max][0] = true;
        grid[max][max] = true;
    }
    grid.into_iter().flatten().filter(|x| *x).count() as i32
}
