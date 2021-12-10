#[inline]
fn error_score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

#[inline]
fn repair_score(c: char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!(c),
    }
}

#[inline]
fn is_open_bracket(c: char) -> bool {
    matches!(c, '(' | '[' | '{' | '<')
}

#[inline]
fn is_matching_bracket(co: char, cc: char) -> bool {
    match co {
        '(' => cc == ')',
        '[' => cc == ']',
        '{' => cc == '}',
        '<' => cc == '>',
        _ => false,
    }
}

fn part1(inp: &[&str]) -> usize {
    let mut stack = Vec::new();
    let mut ret = 0;
    for line in inp {
        stack.clear();
        for c in line.chars() {
            if is_open_bracket(c) {
                stack.push(c);
            } else if !is_matching_bracket(stack.pop().unwrap_or('x'), c) {
                ret += error_score(c);
                break;
            }
        }
    }
    ret
}

fn part2(inp: &[&str]) -> usize {
    let mut stack = Vec::new();
    let mut scores = Vec::new();
    'line: for line in inp {
        stack.clear();
        for c in line.chars() {
            if is_open_bracket(c) {
                stack.push(c);
            } else if !is_matching_bracket(stack.pop().unwrap_or('x'), c) {
                // Line is corrupted, we don't care
                continue 'line;
            }
        }

        scores.push(
            stack
                .iter()
                .rev()
                .map(|c| repair_score(*c))
                .fold(0, |a, b| a * 5 + b),
        );
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    let input = include_str!("input.txt").lines().collect::<Vec<&str>>();
    let t = std::time::Instant::now();
    let p1 = part1(&input);
    let t_p1 = (std::time::Instant::now() - t).as_secs_f64() * 1e6;
    let t = std::time::Instant::now();
    let p2 = part2(&input);
    let t_p2 = (std::time::Instant::now() - t).as_secs_f64() * 1e6;
    println!("Part 1: {:10} (took {:6.2} μs)", p1, t_p1);
    println!("Part 2: {:10} (took {:6.2} μs)", p2, t_p2);

    //    cargo run --release
    // Part 1:     216297 (took  62.58 μs)
    // Part 2: 2165057169 (took  73.43 μs)

}
