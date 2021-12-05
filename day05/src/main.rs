use regex::Regex;

const SIZE: isize = 1000;
type Line = ((isize, isize), (isize, isize));

fn parse_input() -> Option<Vec<Line>>{
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let cap = re.captures(line)?
                .iter()
                .skip(1)
                .map(|c| isize::from_str_radix(c?.as_str(), 10).ok())
                .collect::<Option<Vec<_>>>()?;
            Some(((cap[0], cap[1]), (cap[2], cap[3])))
        })
        .collect::<Option<Vec<_>>>()
}

fn draw_line(buf: &mut [u8], line: &Line) {
    let (s, e) = (line.0, line.1);
    // This factor handles both direction / angle
    let facx = (e.0 - s.0 ).signum();
    let facy = (e.1 - s.1 ).signum();
    let len = std::cmp::max((s.0 - e.0).abs(), (s.1 - e.1).abs());

    for i in 0..len+1 {
        let x = s.0 + facx * i;
        let y = s.1 + facy * i;
        let idx = (y * SIZE + x) as usize;
        buf[idx] = buf[idx].checked_add(1).expect("int overflow");
    }
}

fn solve<'a, I>(buf: &mut [u8], line_iter: I) -> usize 
where
    I: Iterator<Item = &'a Line>
{
    for line in line_iter {
        draw_line(buf, &line);
    }
    buf.iter().filter(|&&c| c > 1).count()
}

// Part 1: 6225
fn part1(buf: &mut [u8], lines: &Vec<Line>) {
    let hor = lines.iter()
        .filter(|line| line.0.1 == line.1.1 || line.0.0 == line.1.0);
    println!("Part 1: {}", solve(buf, hor));
}

// Part 2: 22116
fn part2(buf: &mut [u8], lines: &Vec<Line>) {
    let diag = lines.iter()
        .filter(|line| line.0.1 != line.1.1 && line.0.0 != line.1.0);
    println!("Part 2: {}", solve(buf, diag));
}

fn main() {
    // Guess what? We're just gonna use a 1MB stack buffer for that
    // sweet sweet O(1) access time, without needing to hash on each
    // lookup.

    let mut buf = [0u8; (SIZE*SIZE) as usize];
    let lines = parse_input().expect("Failed to parse input");
    part1(&mut buf, &lines);
    part2(&mut buf, &lines);
}
