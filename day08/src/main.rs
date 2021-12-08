type Note = (Vec<&'static str>, Vec<&'static str>);

fn parse_input() -> Option<Vec<Note>> {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let (patterns, values) = line.split_once('|')?;
            Some((
                patterns.split_whitespace().collect::<Vec<&str>>(),
                values.split_whitespace().collect::<Vec<&str>>(),
            ))
        })
        .collect::<Option<Vec<Note>>>()
}

fn part1(notes: &[Note]) -> usize {
    notes
        .iter()
        .map(|(_, values)| {
            values
                .iter()
                .map(|val| match val.len() {
                    2 => 1,
                    4 => 1,
                    3 => 1,
                    7 => 1,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn segment_value(n: usize) -> Option<usize> {
    match n {
        0b1110111 => Some(0),
        0b0100100 => Some(1),
        0b1011101 => Some(2),
        0b1101101 => Some(3),
        0b0101110 => Some(4),
        0b1101011 => Some(5),
        0b1111011 => Some(6),
        0b0100101 => Some(7),
        0b1111111 => Some(8),
        0b1101111 => Some(9),
        _ => None,
    }
}

#[inline(always)]
fn idx(c: char) -> usize {
    c as usize - 'a' as usize
}

fn part2(notes: &[Note]) -> usize {
    // Re-use the same set allocation
    let mut set = std::collections::HashSet::new();
    let mut ret = 0usize;
    for note in notes {
        // As we progress, add known mappings to this lut.
        // For example, a = 0 ==> Some('b')
        //     aaaa      0000             1
        //    b    c    1    2         2    4
        //    b    c    1    2            8
        //     dddd  ->  3333    ->   16    32
        //    e    f    4    5           64
        //    e    f    4    5
        //     gggg      6666
        //
        let mut lut = [None; 7];
        let mut lut_inv = [None; 7];
        set.clear();

        let one;
        let four;
        let mut five;
        let mut six;
        let seven;
        let eight;

        five = None;

        // First pass, find a 1 and a 7, use them to map `0000`
        one = note.0.iter().find(|a| a.len() == 2).unwrap();
        one.chars().for_each(|c| {
            set.insert(c);
        });
        seven = note.0.iter().find(|a| a.len() == 3).unwrap();
        seven.chars().for_each(|c| {
            if set.insert(c) {
                lut[idx(c)] = Some(0);
                lut_inv[0] = Some(c);
            }
        });

        // Next, find 6, so we can map `cc` and `ff`
        for candidate in note.0.iter().filter(|a| a.len() == 6) {
            set.clear();
            one.chars().for_each(|c| {
                set.insert(c);
            });
            let count: usize = candidate.chars().filter(|&c| set.insert(c)).count();
            if count == 5 {
                // If we mapped 5 new segments, this must be a 6
                set.clear();
                six = candidate;
                six.chars().for_each(|c| {
                    set.insert(c);
                });
                for c in one.chars() {
                    if set.insert(c) {
                        lut[idx(c)] = Some(2);
                        lut_inv[2] = Some(c);
                    } else {
                        lut[idx(c)] = Some(5);
                        lut_inv[5] = Some(c);
                    }
                }
            }
        }

        // We now have the tools to distinguish 5 and 2
        // The 2 can be used to map in `bb` with 4
        four = note.0.iter().find(|a| a.len() == 4).unwrap();
        for candidate in note.0.iter().filter(|a| a.len() == 5) {
            set.clear();
            set.insert(lut_inv[2].unwrap());
            let count: usize = candidate.chars().filter(|&c| set.insert(c)).count();
            if count == 4 {
                // We've found a 2
                set.insert(lut_inv[5].unwrap());
                for c in four.chars() {
                    if set.insert(c) {
                        lut[idx(c)] = Some(1);
                        lut_inv[1] = Some(c);
                    }
                }
            } else if count == 5 {
                // We've found a 5
                // (This code path is not strictly guaranteed, so `five` is an Option)
                five = Some(candidate);
            }
        }

        // We can now easily map `dddd` in as well.
        set.clear();
        lut_inv.iter().for_each(|x| {
            x.and_then(|c| Some(set.insert(c)));
        });
        for c in four.chars() {
            if set.insert(c) {
                lut[idx(c)] = Some(3);
                lut_inv[3] = Some(c);
            }
        }

        // Next, add 5 to the working set, resulting in one new addition.
        for c in five.unwrap().chars() {
            if set.insert(c) {
                lut[idx(c)] = Some(6);
                lut_inv[6] = Some(c);
            }
        }

        // One char left, we could use process of elimination here, but in the
        // spirit of coherence checking, we'll use 8 instead.
        eight = note.0.iter().find(|a| a.len() == 7).unwrap();
        set.clear();
        lut_inv.iter().for_each(|x| {
            x.and_then(|c| Some(set.insert(c)));
        });
        for c in eight.chars() {
            if set.insert(c) {
                lut[idx(c)] = Some(4);
                lut_inv[4] = Some(c);
            }
        }

        ret += note
            .1
            .iter()
            .rev()
            .enumerate()
            .map(|(i, digit)| {
                segment_value(digit.chars().map(|c| 1 << lut[idx(c)].unwrap()).sum()).unwrap()
                    * 10usize.pow(i as u32)
            })
            .sum::<usize>();
    }
    ret
}

// Alternate solve, using brute force to find the first applicable mapping
fn part2_brute(notes: &[Note]) -> usize {
    use itertools::Itertools;
    let mut ret = 0;
    for note in notes {
        let values = [0, 1, 2, 3, 4, 5, 6];
        let lut = values.iter().permutations(values.len()).find(|lut| {
            note.0.iter().all(|pattern| {
                let v = pattern.chars().map(|c| 1 << lut[idx(c)]).sum::<usize>();
                segment_value(v).is_some()
            })
        });

        let lut = lut.expect("Failed to find a lut");

        ret += note
            .1
            .iter()
            .rev()
            .enumerate()
            .map(|(i, digit)| {
                segment_value(digit.chars().map(|c| 1 << lut[idx(c)]).sum()).unwrap()
                    * 10usize.pow(i as u32)
            })
            .sum::<usize>();
    }
    ret
}

fn main() {
    let input = parse_input().expect("Input was not valid.");
    let t = std::time::Instant::now();
    let part1 = part1(&input);
    let time_p1 = t.elapsed().as_secs_f64() * 1e6;
    let t = std::time::Instant::now();
    let part2 = part2(&input);
    let time_p2 = t.elapsed().as_secs_f64() * 1e6;
    let t = std::time::Instant::now();
    let part2_brute = part2_brute(&input);
    let time_p2_brute = t.elapsed().as_secs_f64() * 1e3;
    println!("Part 1 : {:10} (took {:9.3} μs)", part1, time_p1);
    println!("Part 2 : {:10} (took {:9.3} μs)", part2, time_p2);
    println!(
        "Part 2b: {:10} (took {:9.3} ms)",
        part2_brute, time_p2_brute
    );

    // Trial run:
    // Part 1 :        362 (took     1.370 μs)
    // Part 2 :    1020159 (took   419.134 μs)
    // Part 2b:    1020159 (took    29.356 ms)
}
