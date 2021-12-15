use aoc::*;


struct Board {
    positions: [[usize; 5]; 5],
    marked: [[bool; 5]; 5],
    won: bool
}


impl Board {
    fn fill(&mut self, numbers: Vec<Vec<usize>>) {
        for (x, row) in numbers.iter().enumerate() {
            for (y, column) in row.iter().enumerate() {
                self.positions[x][y] = *column;
            }
        }
    }

    fn check_called(&mut self, number: usize) -> bool {
        for x in 0..5 {
            for y in 0..5 {
                if self.positions[x][y] == number {
                    self.marked[x][y] = true;
                    break;
                }
            }
        }
        for x in 0..5 {
            let mut found = true;
            for y in 0..5 {
                found &= self.marked[x][y];
                if !found {
                    break;
                }
            }

            if found {
                self.won = true;
                return true;
            }
        }

        for x in 0..5 {
            let mut found = true;
            for y in 0..5 {
                found &= self.marked[y][x];
                if !found {
                    break;
                }
            }

            if found {
                self.won = true;
                return true;
            }
        }

        false
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

    let boards = inputs[1..]
                    .iter()
                    .map(|l| l.split_whitespace()
                        .map(|number| number.parse().unwrap())
                        .collect::<Vec<usize>>())
                    .collect::<Vec<Vec<usize>>>();

    let mut all_boards: Vec<Board> = Vec::new();
    for b in boards.chunks(5) {
        let mut board = Board{positions: [[0; 5]; 5], marked: [[false; 5]; 5], won: false};
        board.fill(b.to_vec());
        all_boards.push(board);
    }
    (numbers, all_boards)
}


fn check_boards(numbers: Vec<usize>, mut all_boards: Vec<Board>) -> Vec<usize> {
    let mut winners: Vec<usize> = Vec::new();

    for n in numbers {
        for b in &mut all_boards {
            if b.won {
                continue;
            }
            if b.check_called(n) {
                let mut sum = 0;
                for x in 0..5 {
                    for y in 0..5 {
                        if !b.marked[x][y] {
                            sum += b.positions[x][y];
                        }
                    }
                }
                winners.push(n * sum);
            }
        }
    }
    winners
}


fn part_1() -> usize {
    let all_boards = make_boards(get_inputs());
    check_boards(all_boards.0, all_boards.1)[0]
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
