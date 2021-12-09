use std::collections::{BTreeSet, VecDeque};

fn parse_input() -> Vec<Vec<u8>> {
    include_str!("input.txt")
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>()
}

fn part1(input: &[Vec<u8>]) -> (Vec<(usize, usize)>, usize) {
    let width = input[0].len();
    let heigth = input.len();
    let mut lowpoints = Vec::new();
    for x in 0..width {
        for y in 0..heigth {
            let lowp = [(0, 1), (0, -1), (1, 0), (-1, 0)].iter().all(|(dx, dy)| {
                let px = (x as isize + dx) as usize;
                let py = (y as isize + dy) as usize;
                match input.get(py).and_then(|row| row.get(px)) {
                    Some(e) => e > &input[y][x],
                    None => true,
                }
            });
            if lowp {
                lowpoints.push((x, y));
            }
        }
    }

    let res = lowpoints
        .iter()
        .map(|&(x, y)| input[y][x] as usize + 1)
        .sum();
    (lowpoints, res)
}

fn part2(input: &[Vec<u8>], lowpoints: &[(usize, usize)]) -> usize {
    let mut seen: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut todo: VecDeque<(usize, usize)> = VecDeque::new();

    let mut basin_sizes = Vec::new();

    for lowp in lowpoints {
        seen.clear();
        todo.clear();
        seen.insert(*lowp);
        todo.push_front(*lowp);

        while let Some((x, y)) = todo.pop_front() {
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let px = x as isize + dx;
                let py = y as isize + dy;
                if px < 0 || py < 0 {
                    continue;
                }
                // Rebind to usizes
                let px = px as usize;
                let py = py as usize;

                // Add an item to the work queue if it is grequal to the center element,
                // but not equal to 9.

                let add = match input.get(py).and_then(|row| row.get(px)) {
                    Some(e) => *e >= input[y][x] && *e < 9,
                    None => false,
                };
                if add && seen.insert((px, py)) {
                    todo.push_front((px, py));
                }
            }
        }

        basin_sizes.push(seen.len());
    }
    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

fn main() {
    let input = parse_input();
    let t = std::time::Instant::now();
    let (lowp, p1) = part1(&input);
    let t_p1 = (std::time::Instant::now() - t).as_secs_f64() * 1e6;
    let t = std::time::Instant::now();
    let p2 = part2(&input, &lowp);
    let t_p2 = (std::time::Instant::now() - t).as_secs_f64() * 1e6;
    println!("Part 1: {:10} (took {:6.2} μs)", p1, t_p1);
    println!("Part 2: {:10} (took {:6.2} μs)", p2, t_p2);

    // cargo run --release:
    // Part 1:        506 (took  77.62 μs)
    // Part 2:     931200 (took 698.95 μs)
}
