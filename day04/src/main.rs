const CARD_SIZE: usize = 5;

#[derive(Debug, Clone)]
struct BingoCard {
    size : usize,
    grid : Vec<Vec<usize>>,
    chips: Vec<Vec<bool>>,
}

impl BingoCard {
    pub fn new(str_card: &[&str]) -> Option<Self> {
        let grid = str_card
            .iter()
            .map(|line| {
                line
                    .split_whitespace()
                    .map(|x|  usize::from_str_radix(x, 10))
                    .collect::<Result<Vec<usize>, _>>()
            })
            .collect::<Result<Vec<Vec<_>>, _>>().ok()?;
        Some(BingoCard {
            size : CARD_SIZE,
            grid : grid,
            chips: vec![vec![false; CARD_SIZE]; CARD_SIZE],
        })
    }

    // Marks a number on the bingo card and returns a result in case of bingo.
    pub fn add_check(&mut self, number: usize) -> Option<usize> {
        // Find the x/y of the new number
        let pos = self.grid.iter().flatten().enumerate()
            .filter(|&(_, x)| *x == number).map(|x| x.0).next()?;
        let (x, y) = (pos % self.size, pos / self.size);

        // If we found one, place a chip on it
        self.chips[y][x] = true;

        // Next, scan the x/y of this new chip to check if it completed
        // a row or column. (remember, no diagonals)
        let res = self.chips[y].iter().all(|&e| e) || self.chips.iter().all(|col| col[x]);

        // If either method causes a bingo, calculate the result.
        if res {
            let it = self.grid.iter().flatten().zip(self.chips.iter().flatten());
            let sum = it.filter(|&(_, c)| !c).map(|(n, _)| *n).sum::<usize>();
            return Some(sum * number);
        }
        None
    }
}

// Part 1: 60368
fn part1(numbers: &Vec<usize>, cards: &mut Vec<BingoCard>) {
    for number in numbers {
        for card in &mut *cards {
            if let Some(res) = card.add_check(*number) {
                println!("Part 1: {}", res);
                return;
            }
        }
    }
    unreachable!();
}

// Part 2: 17435
fn part2(numbers: &Vec<usize>, cards: &mut Vec<BingoCard>) {
    for number in numbers {
        let mut offset = 0;
        for i in 0..cards.len() {
            let idx = i - offset;
            if let Some(res) = cards[idx].add_check(*number) {
                if cards.len() == 1 {
                    println!("Part 2: {}", res);
                    return;
                }
                cards.remove(idx);
                offset += 1;
            }
        }
    }
    unreachable!();
}

fn parse_input() -> Option<(Vec<usize>, Vec<BingoCard>)> {
    let inp = include_str!("input.txt")
        .lines()
        .collect::<Vec<&str>>();

    // Parse the first line with called bingo numbers
    let numbers = inp[0]
        .split(",")
        .map(|x|  usize::from_str_radix(x, 10))
        .collect::<Result<Vec<usize>, _>>().ok()?;
    // Next up, cards
    let cards = inp[1..]
        .chunks(1+CARD_SIZE)
        .map(|chunk| BingoCard::new(&chunk[1..]))
        .collect::<Option<Vec<BingoCard>>>()?;

    Some((numbers, cards))
}

fn main() {
    let (numbers, cards) = parse_input().expect("Failed to parse input");
    part1(&numbers, &mut cards.clone());
    part2(&numbers, &mut cards.clone());
}
