#[derive(Debug, Clone)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}

fn take_node(iter: &mut dyn Iterator<Item = &str>) -> Node {
    let child_num: u32 = iter.next().unwrap().parse().unwrap();
    let metadata_num: u32 = iter.next().unwrap().parse().unwrap();
    let children: Vec<Node> = (0..child_num).map(|_| take_node(iter)).collect();
    let metadata: Vec<u32> = iter
        .take(metadata_num as usize)
        .map(|x| x.parse().unwrap())
        .collect();
    Node { children, metadata }
}

fn get_sum_1(n: Node) -> u32 {
    n.metadata.iter().sum::<u32>() + n.children.into_iter().map(get_sum_1).sum::<u32>()
}

#[aoc(day08, part1)]
pub fn part1(input: &str) -> u32 {
    let mut data = input.split(' ');
    let a = take_node(&mut data);

    get_sum_1(a)
}

fn get_sum_2(n: Node) -> u32 {
    if n.children.is_empty() {
        n.metadata.iter().sum()
    } else {
        n.metadata
            .iter()
            .filter_map(|c| n.children.get(*c as usize - 1))
            .map(|x| get_sum_2(x.clone()))
            .sum()
    }
}

#[aoc(day08, part2)]
pub fn part2(input: &str) -> u32 {
    // let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
    let mut data = input.split(' ');
    let a = take_node(&mut data);

    get_sum_2(a)
}
