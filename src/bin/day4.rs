use aoc::*;


struct Board {
    positions: [[(usize, bool); 5]; 10]
}


impl Board {
    fn new(numbers: &[Vec<usize>]) -> Self {
        let mut board = Board{positions: [[(0, false); 5]; 10]};
        for (r, row) in numbers.iter().enumerate() {
            for (c, column) in row.iter().enumerate() {
                board.positions[r][c].0 = *column;
                board.positions[c + 5][r].0 = *column;
            }
        }
        board
    }

    fn is_winner(&self) -> bool {
        self.positions
            .iter()
            .any(|&row|
                row
                    .iter()
                    .all(|&c| c.1 == true)
            )
    }

    fn mark_called(&mut self, number: usize) {
        for r in 0..10 {
            match self.positions[r].iter().position(|&x| x.0 == number) {
                Some(i) => self.positions[r][i].1 = true,
                None => ()
            }
        }
    }

    fn sum_non_marked(&self) -> usize {
        self.positions[0..5]
            .iter()
            .map(|r| r
                .iter()
                .filter(|c| !c.1)
                .map(|c| c.0)
                .sum::<usize>()
            ).sum()
    }
}


fn get_inputs() -> Vec<&'static str> {
    include_str!("../../input/day4.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<&'static str>>()
}


fn make_boards(inputs: Vec<&'static str>) -> (Vec<usize>, Vec<Board>) {
    let numbers = inputs[0]
                    .split(',')
                    .map(|s| s.trim().parse().unwrap())
                    .collect::<Vec<usize>>();

    let all_boards: Vec<Board> = inputs[1..]
        .iter()
        .map(|l| l.split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect())
        .collect::<Vec<Vec<usize>>>()
        .chunks(5)
        .map(|b| Board::new(&b.to_vec()))
        .collect();

    (numbers, all_boards)
}


fn check_boards(numbers: Vec<usize>, mut all_boards: Vec<Board>) -> Vec<usize> {
    let mut winners: Vec<usize> = Vec::new();

    for n in numbers {
        all_boards.retain(|b| !b.is_winner());
        for b in &mut all_boards {
            b.mark_called(n);
            if b.is_winner() {
                winners.push(n * b.sum_non_marked());
            }
        }
    }
    winners
}


fn part_1() -> usize {
    let all_boards = make_boards(get_inputs());
    *check_boards(all_boards.0, all_boards.1).first().unwrap()
}


fn part_2() -> usize {
    let all_boards = make_boards(get_inputs());
    *check_boards(all_boards.0, all_boards.1).last().unwrap()
}


fn main() -> Result<()> {
    println!("Part 1 {}", part_1());
    println!("Part 2 {}", part_2());
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert!(part_1() == 8_136);
    }

    
    #[test]
    fn test_part_2() {
        assert!(part_2() == 12_738);
    }
}
