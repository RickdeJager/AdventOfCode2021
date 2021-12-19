fn part1(mut grid: Vec<Vec<u8>>) -> usize {
    let (w, h) = (grid[0].len(), grid.len());

    let mut ctr = 0;
    for _ in 0..100 {
        // Increase each field by 1
        grid.iter_mut().flatten().for_each(|c| *c += 1);

        // Settle flashes
        loop {
            let mut flashed = false;
            for x in 0..w {
                for y in 0..h {
                    if grid[y][x] == 10 {
                        // increase s.t. this one won't flash twice
                        grid[y][x] += 1;
                        // flash neighbors
                        let (sy, ey) = (y as isize - 1, y as isize + 2);
                        let (sx, ex) = (x as isize - 1, x as isize + 2);
                        for iy in sy..ey {
                            for ix in sx..ex {
                                // cool hack or horrible idea?
                                grid.get_mut(iy as usize).and_then(|col| {
                                    col.get_mut(ix as usize).map(|c| {
                                        flashed = true;
                                        if *c != 10 {
                                            *c += 1;
                                        }
                                    })
                                });
                            }
                        }
                    }
                }
            }
            if !flashed {
                break;
            }
        }

        // Tally up flashes
        grid.iter_mut()
            .flatten()
            .filter(|&&mut c| c > 9)
            .for_each(|c| {
                ctr += 1;
                *c = 0
            });
    }

    ctr
}

fn part2(mut grid: Vec<Vec<u8>>) -> usize {
    let (w, h) = (grid[0].len(), grid.len());
    let max = grid.iter().flatten().count();

    for step in 1.. {
        // Increase each field by 1
        grid.iter_mut().flatten().for_each(|c| *c += 1);

        // Settle flashes
        loop {
            let mut flashed = false;
            for x in 0..w {
                for y in 0..h {
                    if grid[y][x] == 10 {
                        // increase s.t. this one won't flash twice
                        grid[y][x] += 1;
                        // flash neighbors
                        let (sy, ey) = (y as isize - 1, y as isize + 2);
                        let (sx, ex) = (x as isize - 1, x as isize + 2);
                        for iy in sy..ey {
                            for ix in sx..ex {
                                // cool hack or horrible idea?
                                grid.get_mut(iy as usize).and_then(|col| {
                                    col.get_mut(ix as usize).map(|c| {
                                        flashed = true;
                                        if *c != 10 {
                                            *c += 1;
                                        }
                                    })
                                });
                            }
                        }
                    }
                }
            }
            if !flashed {
                break;
            }
        }

        // Tally up flashes
        let count = grid
            .iter_mut()
            .flatten()
            .filter(|&&mut c| c > 9)
            .map(|c| *c = 0)
            .count();
        if count == max {
            return step;
        }
    }

    unreachable!();
}

fn main() {
    let inp = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect::<Vec<Vec<_>>>();

    println!("Part 1: {}", part1(inp.clone()));
    println!("Part 2: {}", part2(inp));
}
