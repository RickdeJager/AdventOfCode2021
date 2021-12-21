use std::collections::BTreeSet;

#[derive(Copy, Clone, Debug)]
enum Fold {
    Y(isize),
    X(isize),
}

fn parse_input() -> Option<(BTreeSet<(isize, isize)>, Vec<Fold>)> {
    let it = include_str!("input.txt").lines();

    // Parse coords
    let coords = it
        .clone()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut tmp = line.split(',');
            Some((
                tmp.next()?.parse::<isize>().ok()?,
                tmp.next()?.parse::<isize>().ok()?,
            ))
        })
        .collect::<Option<BTreeSet<(isize, isize)>>>()?;

    // Parse folds
    let folds = it
        .skip(coords.len() + 1)
        .map(|line| {
            let mut parts = line.strip_prefix("fold along ")?.split('=');
            let (dir, num) = (parts.next()?, parts.next()?.parse::<isize>().ok()?);
            Some(match dir {
                "x" => Fold::X(num),
                "y" => Fold::Y(num),
                _ => None?,
            })
        })
        .collect::<Option<Vec<Fold>>>()?;
    Some((coords, folds))
}

fn solve(coords: &BTreeSet<(isize, isize)>, folds: &[Fold]) -> BTreeSet<(isize, isize)> {
    let mut set1 = coords.clone();
    let mut set2 = BTreeSet::new();
    for fold in folds {
        for coord in set1 {
            match *fold {
                Fold::X(value) => {
                    let mut c = coord;
                    if coord.0 > value {
                        c.0 -= (c.0 - value) * 2;
                    }
                    set2.insert(c);
                }
                Fold::Y(value) => {
                    let mut c = coord;
                    if coord.1 > value {
                        c.1 -= (c.1 - value) * 2;
                    }
                    set2.insert(c);
                }
            }
        }
        set1 = set2.clone();
        set2.clear();
    }
    set1
}

fn part1(coords: &BTreeSet<(isize, isize)>, folds: &[Fold]) -> usize {
    solve(coords, &folds[..1]).len()
}

fn part2(coords: &BTreeSet<(isize, isize)>, folds: &[Fold]) {
    let set = solve(coords, folds);
    let vec = set.iter().collect::<Vec<_>>();
    let max_x = *vec.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *vec.iter().map(|(_, y)| y).max().unwrap();

    for y in 0..max_y + 1 {
        for x in 0..max_x + 1 {
            if set.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    let (coords, folds) = parse_input().expect("Invalid input");
    println!("Part 1: {}", part1(&coords, &folds));
    println!("Part 2:");
    part2(&coords, &folds);
}
