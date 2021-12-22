use std::collections::HashMap;

fn parse_input() -> Option<(Vec<char>, HashMap<(char, char), char>)> {
    let mut it = include_str!("input.txt").lines();
    let initial = it.next()?.chars().collect();
    let map = it
        .clone()
        .skip(1)
        .map(|line| {
            let mut parts = line.chars();
            let key = (parts.next()?, parts.next()?);
            let value = parts.nth(4)?;
            Some((key, value))
        })
        .collect::<Option<HashMap<_, _>>>()?;
    Some((initial, map))
}

fn solve(line: &[char], rule_map: &HashMap<(char, char), char>, steps: usize) -> usize {
    let mut pair_map = HashMap::new();
    line.windows(2).map(|c| (c[0], c[1])).for_each(|pair| {
        *pair_map.entry(pair).or_insert(0) += 1;
    });

    for _ in 0..steps {
        let mut temp_map = HashMap::new();
        for (&pair, &count) in &pair_map {
            if count == 0 {
                continue;
            }
            if let Some(&value) = rule_map.get(&pair) {
                // Add two new pairs
                let (left, right) = ((pair.0, value), (value, pair.1));
                *temp_map.entry(left).or_insert(0) += count;
                *temp_map.entry(right).or_insert(0) += count;
            }
        }
        pair_map = temp_map;
    }

    let mut counts = HashMap::new();
    counts.insert(line[0], 1);
    for (k, v) in pair_map {
        *counts.entry(k.1).or_insert(0) += v;
    }
    let (mut min, mut max) = ((usize::MAX, '.'), (0, '.'));
    for (k, v) in counts {
        if v < min.0 {
            min.0 = v;
            min.1 = k;
        }
        if v > max.0 {
            max.0 = v;
            max.1 = k;
        }
    }

    max.0 - min.0
}

fn main() {
    let (line, map) = parse_input().expect("Failed to parse input");
    println!("Part 1: {}", solve(&line, &map, 10));
    println!("Part 2: {}", solve(&line, &map, 40));
}
