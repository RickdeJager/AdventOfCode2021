use std::collections::{BTreeSet, HashMap};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Cave {
    // id -> 0
    Start(usize),
    // id -> 1
    End(usize),
    // id
    Big(usize),
    // id
    Small(usize),
}
impl Cave {
    fn id(self) -> usize {
        match self {
            Cave::Start(id) => id,
            Cave::End(id) => id,
            Cave::Big(id) => id,
            Cave::Small(id) => id,
        }
    }
    fn from_label(label: &str, id: usize) -> Option<Self> {
        Some(match label.chars().next()?.is_uppercase() {
            true => Cave::Big(id),
            false => Cave::Small(id),
        })
    }
}

#[derive(Default, Debug)]
struct Adj {
    data: Vec<Vec<bool>>,
}

impl Adj {
    fn insert_cave<'a>(&mut self, cave: &'a Cave) -> Option<&'a Cave> {
        // Unwrap the cave's ID from the enum
        let id = cave.id();
        // Check if this cave is new
        if id >= self.data.len() {
            assert!(id == self.data.len());
            for col in &mut self.data {
                col.push(false);
            }
            self.data.push(vec![false; id + 1]);
            return Some(cave);
        }
        None
    }

    // In case one of the nodes is invalid, this returns none without touching either, since
    // the grid is square, causing the first get_mut chain to fail before assignement.
    fn insert_edge(&mut self, src: &Cave, dst: &Cave) -> Option<()> {
        let (src_id, dst_id) = (src.id(), dst.id());
        *self.data.get_mut(src_id)?.get_mut(dst_id)? = true;
        *self.data.get_mut(dst_id)?.get_mut(src_id)? = true;
        Some(())
    }

    fn adjacent(&self, src: &Cave, dst: &Cave) -> Option<bool> {
        let (src_id, dst_id) = (src.id(), dst.id());
        Some(*self.data.get(src_id)?.get(dst_id)?)
    }
}

fn parse_input() -> Option<(Adj, BTreeSet<Cave>)> {
    let mut ctr = 2;
    let mut label_map = HashMap::new();
    let mut cave_set = BTreeSet::new();
    let mut adj_matrix = Adj::default();
    label_map.insert("start", Cave::Start(0));
    label_map.insert("end", Cave::End(1));
    adj_matrix.insert_cave(&Cave::Start(0));
    adj_matrix.insert_cave(&Cave::End(1));
    cave_set.insert(Cave::End(1));

    include_str!("input.txt").lines().try_for_each(|line| {
        let mut tmp = line.trim().split('-');
        let from = tmp.next().unwrap();
        let dest = tmp.next().unwrap();
        // Insert both nodes
        for loc in &[from, dest] {
            if !label_map.contains_key(loc) {
                let cave = Cave::from_label(loc, ctr)?;
                cave_set.insert(cave);
                label_map.insert(loc, cave);
                adj_matrix.insert_cave(&cave);
                ctr += 1
            }
        }
        // Insert the edge
        adj_matrix.insert_edge(label_map.get(from)?, label_map.get(dest)?)?;
        Some(())
    })?;

    Some((adj_matrix, cave_set))
}

fn solve(
    adj: &Adj,
    working_set: &mut BTreeSet<Cave>,
    current_cave: &Cave,
    revisit: Option<(Cave, bool)>,
) -> usize {
    // Base case
    if let Cave::End(_) = current_cave {
        // Only count this as a new path in the following cases:
        // * Nothing was set to be revisited
        // * A revisit target was set, and actually revisited
        return match revisit {
            Some((_, false)) => 0,
            _ => 1,
        };
    }

    let mut ret = 0;
    let mut s = working_set.clone();
    for next_cave in working_set.iter() {
        if adj.adjacent(current_cave, next_cave).unwrap() {
            let mut new_revisit = revisit;
            if let Cave::Small(_) = next_cave {
                match &mut new_revisit {
                    None => {
                        // Branch off, select this cave as the one to be revisited
                        // before removing it from the set
                        ret += solve(adj, &mut s, next_cave, Some((*next_cave, false)));
                    }
                    Some((cave, flag)) => {
                        if cave == next_cave {
                            *flag = true;
                        }
                    }
                }
                s.remove(next_cave);
            }
            ret += solve(adj, &mut s, next_cave, new_revisit);
            s.insert(*next_cave);
        }
    }
    ret
}

fn part1(adj: &Adj, cave_set: &BTreeSet<Cave>) -> usize {
    let mut scratch = cave_set.clone();
    solve(
        adj,
        &mut scratch,
        &Cave::Start(0),
        Some((Cave::Start(0), true)),
    )
}

fn part2(adj: &Adj, cave_set: &BTreeSet<Cave>) -> usize {
    let mut scratch = cave_set.clone();
    solve(adj, &mut scratch, &Cave::Start(0), None)
}

fn main() {
    let (adj, cave_map) = parse_input().expect("Failed to parse input");
    let t = std::time::Instant::now();
    let p1 = part1(&adj, &cave_map);
    let e1 = t.elapsed().as_secs_f64();
    let t = std::time::Instant::now();
    let p2 = part2(&adj, &cave_map);
    let e2 = t.elapsed().as_secs_f64();
    println!("Part 1: {:10} (took {:3.3} ms)", p1, e1 * 1e3); 
    println!("Part 2: {:10} (took {:3.3} ms)", p2, e2 * 1e3);

    // cargo run --release
    // Part 1:       4691 (took 1.598 ms)
    // Part 2:     140718 (took 60.876 ms)
}

