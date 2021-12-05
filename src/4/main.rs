use std::str::FromStr;

#[derive(Debug)]
struct Board<const N: usize> {
    data: [[(u16, bool); N]; N],
}

impl<const N: usize> Default for Board<N> {
    fn default() -> Self {
        Self {
            data: [[(0u16, false); N]; N],
        }
    }
}

impl<const N: usize> FromStr for Board<N> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Self::default();

        for (i, s) in s.lines().enumerate() {
            if i >= N {
                return Err("too many rows in input".to_string());
            }

            let values = s
                .split_whitespace()
                .map(|v| v.parse::<u16>())
                .collect::<Result<Vec<u16>, _>>()
                .map_err(|e| e.to_string())?;

            for (i, v) in result.data[i].iter_mut().enumerate() {
                v.0 = match values.get(i) {
                    Some(it) => *it,
                    None => return Err("not enough values in row".to_string()),
                }
            }
        }

        Ok(result)
    }
}

impl<const N: usize> Board<N> {
    fn set(&mut self, val: u16) {
        let found = self.data.iter_mut().flatten().find(|v| v.0 == val);
        if let Some(v) = found {
            v.1 = true
        }
    }

    fn score(&self, last: u16) -> u32 {
        if last == 0 {
            println!("last invalid {}", last);
        }
        let sum: u16 = self
            .data
            .iter()
            .flatten()
            .map(|v| if !v.1 { v.0 } else { 0 })
            .sum();

        if sum == 0 {
            println!("sum invalid: {}", sum);
        }
        sum as u32* last as u32
    }

    fn has_bingo(&self) -> bool {
        for i in 0..N {
            let rowbingo = self.data[i].iter().all(|row| row.1);
            let colbingo = self.data.iter().all(|row| row[i].1);
            if rowbingo || colbingo {
                return true;
            }
        }

        false
    }
}

fn main() -> Result<(), String> {
    let input = std::fs::read_to_string("src/4/input.txt").expect("failed to read input");

    let mut blocks = input.split("\n\n");
    let values = blocks
        .next()
        .unwrap()
        .split(',')
        .map(|i| i.parse::<u16>().expect("invalid value in input"));

    let mut boards = blocks
        .map(|b| b.parse::<Board<5>>().expect("invalid board input"))
        .collect::<Vec<Board<5>>>();

    let mut winners: Vec<(usize, u32)> = Vec::new();
    for (idx, v) in values.enumerate() {
        for board in &mut boards {
            board.set(v);
        }

        if idx >= 5 {
            for (idx, board) in boards.iter().enumerate() {
                let already_won = winners.iter().any(|won| won.0 == idx);
                if !already_won && board.has_bingo() {
                    println!("Adding board {}", idx);
                    winners.push((idx, board.score(v)));
                }
            }
        }
    }

    println!("{:?}", winners);

    println!("Part 1: winner score: {}", winners[0].1);
    println!("Part 2: winner score: {}", winners.last().unwrap().1);

    Ok(())
}
