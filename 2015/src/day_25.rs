#[aoc(day25, part1)]
pub fn part1(input: &str) -> u64 {
    let input = input.replace(',', "").replace('.', "");
    let mut data = input.split(' ').filter_map(|x| x.parse::<u64>().ok());
    let row = data.next().unwrap();
    let col = data.next().unwrap();
    let mut cur = 20151125;
    for _ in 1..at(row, col) {
        cur = cur * 252533 % 33554393
    }
    cur
}

#[aoc(day25, part2)]
pub fn part2(_: &str) -> &'static str {
    "GG"
}
// row 2981 col 3075
//
//  1  2  3  4  5  6  7
//1 1  3  6  10 15 21 28
//2 2  5  9  14 20 27
//3 4  8  13 19 26
//4 7  12 18 25
//5 11 17 24
//6 16 23
//7 22
//
//
// anywhere in the grid
// x > x + col_of_x + 1
// v
// x + row_of_x + 1
//
// row x, col y => ((y+1)*(y+2))/2 + (x*(x+1))/2 - 1
// row 2, col 2 => 3 + 3 - 1 = 5
// row 4 col 3 => 10+
//

fn at(row: u64, col: u64) -> u64 {
    ((row - 1) * (row - 2)) / 2 + ((col) * (col + 1)) / 2 + col * (row - 1)
}

// (1)        (1+2)        (1+2+3)        (1+2+3+4)
// (1)+1      (1+2)+2      (1+2+3)+3      (1+2+3+4)+4
// (1)+1+2    (1+2)+2+3    (1+2+3)+3+4    (1+2+3+4)+4+5
// (1)+1+2+3  (1+2)+2+3+4  (1+2+3)+3+4+5  (1+2+3+4)+4+5+6
