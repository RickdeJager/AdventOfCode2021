use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};

fn parse_input() -> Vec<Vec<u8>> {
    include_str!("input.txt")
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>()
}

// Helper function to index as options
fn idx<T: Copy>(grid: &[Vec<T>], x: usize, y: usize) -> Option<T> {
    Some(*grid.get(y)?.get(x)?)
}

fn solve(grid: &[Vec<u8>]) -> usize {
    let (w, h) = (grid[0].len(), grid.len());
    let (start, end) = ((0, 0), (w - 1, h - 1));
    let mut distances = vec![vec![usize::MAX; w]; h];
    distances[start.1][start.0] = 0;
    // This is a max heap, but we can wrap all values with
    // std::cmp::Reverse to emulate a min-heap
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), start));
    let mut visited = BTreeSet::new();

    while let Some((cost, coord)) = queue.pop() {
        visited.insert(coord);
        for delta in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let (x, y) = (coord.0 as isize + delta.0, coord.1 as isize + delta.1);
            if x < 0 || y < 0 {
                continue;
            }
            let (x, y) = (x as usize, y as usize);
            if visited.contains(&(x, y)) {
                continue;
            }
            if let Some(neighbor_cost) = idx(grid, x, y) {
                let old_dist = distances[y][x];
                let new_dist = cost.0 + neighbor_cost as usize;
                if new_dist < old_dist {
                    distances[y][x] = new_dist;
                    queue.push((Reverse(new_dist), (x, y)));
                }
            }
        }
    }
    distances[end.1][end.0]
}

fn part2(grid: &[Vec<u8>]) -> usize {
    let (w, h) = (grid[0].len(), grid.len());
    let factor = 5;
    let new_grid = grid
        .iter()
        .cycle()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .cycle()
                .enumerate()
                .take(factor * w)
                .map(|(x, c)| {
                    let mut tmp = *c as usize + x / w + y / h;
                    tmp += tmp / 10;
                    tmp as u8 % 10
                })
                .collect()
        })
        .take(factor * h)
        .collect::<Vec<Vec<u8>>>();

    solve(&new_grid)
}

fn main() {
    let grid = parse_input();
    println!("Part 1: {}", solve(&grid));
    println!("Part 2: {}", part2(&grid));
}
