fn part1(inp: &[isize]) -> isize {
    let max = inp.iter().max().unwrap();
    (0..*max)
        .map(|dest| inp.iter().map(|loc| (dest - loc).abs()).sum())
        .min()
        .unwrap()
}

fn part2(inp: &[isize]) -> isize {
    let max = inp.iter().max().unwrap();
    (0..*max)
        .map(|dest| {
            inp.iter()
                .map(|loc| {
                    let dist = (dest - loc).abs();
                    (dist * (dist + 1)) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|c| c.parse())
        .collect::<Result<Vec<isize>, _>>()
        .expect("invalid input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
