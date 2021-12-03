// Part 1: 3912944

fn get_min_max(grid: &Vec<Vec<u8>>, idx: usize, tie: u8) -> (u8, u8){
    let zeros = grid.iter()
        .filter(|row| row[idx] == 0)
        .count();
    let ones =  grid.iter()
        .filter(|row| row[idx] == 1)
        .count();

    if ones == zeros {
        return (tie, tie)
    }

    if ones > zeros {
        return (0, 1)
    }
    (1, 0)
}

fn part1(inp: &Vec<Vec<u8>>) {
    let len = inp[0].len();
    let res = (0..len)
        .into_iter()
        .map(|idx| {
            get_min_max(inp, idx, 1)
        })
        .collect::<Vec<(u8, u8)>>();
    let eps = res.iter().map(|x| x.0.to_string()).collect::<String>();
    let eps = usize::from_str_radix(&eps, 2).unwrap();
    let gam = res.iter().map(|x| x.1.to_string()).collect::<String>();
    let gam = usize::from_str_radix(&gam, 2).unwrap();
    println!("Part 1: {:?}", gam*eps);
}

fn part2(inp: &Vec<Vec<u8>>) {
    let mut grid = inp.clone();
    let len = grid[0].len();
    for idx in 0..len {
        let (_, max) = get_min_max(&grid, idx, 1);
        grid.retain(|row| {row[idx] == max});

        if grid.len() <= 1 {
            break;
        }
    }
    let ox = grid[0].iter().map(|x| x.to_string()).collect::<String>();
    let ox = usize::from_str_radix(&ox, 2).unwrap();

    let mut grid = inp.clone();
    for idx in 0..len {
        let (min, _) = get_min_max(&grid, idx, 0);
        grid.retain(|row| {row[idx] == min});

        if grid.len() <= 1 {
            break;
        }
    }
    let co2 = grid[0].iter().map(|x| x.to_string()).collect::<String>();
    let co2 = usize::from_str_radix(&co2, 2).unwrap();
    println!("Part 2: {:?}", co2*ox);
}

fn main() {
    // collect the input into an easier format
    let inp = include_str!("input.txt")
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| {(c == '1') as u8})
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();
    part1(&inp);
    part2(&inp);
}
