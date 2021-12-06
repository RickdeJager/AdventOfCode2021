fn solve(fish: &Vec<usize>, days: usize) -> usize {
    let mut buckets = [0usize; 8+1];
    for &f in fish {
        buckets[f] += 1;
    }

    for _ in 0..days {
        // The rotate implicity births buckets[0] number of fish.
        buckets.rotate_left(1);
        // We just need to reset the fish's parent counter as well
        buckets[6] += buckets[8];
    }
    buckets.iter().sum()
}

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|c| usize::from_str_radix(c, 10))
        .collect::<Result<Vec<usize>, _>>()
        .expect("invalid input");
    println!("Part 1: {}", solve(&input, 80));
    println!("Part 1: {}", solve(&input, 256));
}
