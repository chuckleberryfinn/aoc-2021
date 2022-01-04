use aoc::*;
use std::collections::{BinaryHeap, HashSet};


fn get_inputs() -> Vec<Vec<u32>> {
    include_str!("../../input/day9.txt")
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>()
}


fn all_lows(inputs: &[Vec<u32>]) -> Vec<(i32, i32)> {
    inputs
        .iter()
        .enumerate()
        .map(|(i, row)|
            row
                .iter()
                .enumerate()
                .filter(|(j, &column)| !((i > 0 && column >= inputs[i-1][*j])
                                        || (*j > 0 && column >= inputs[i][j-1])
                                        || (i + 1 < inputs.len() && column >= inputs[i+1][*j])
                                        || (j + 1 < inputs[0].len() && column >= inputs[i][j+1]))
                )
                .into_iter()
                .map(|(j, _)| (i as i32, j as i32))
                .collect::<Vec<(i32, i32)>>()
        )
        .flatten()
        .collect()
}


fn part_1() -> u32 {
    let inputs = get_inputs();

    all_lows(&inputs)
        .iter()
        .map(|(x, y)| inputs[*x as usize][*y as usize] + 1)
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}


fn basin(inputs: &[Vec<u32>], start: (i32, i32), seen: &mut HashSet<(i32, i32)>) -> usize {
    if start.0 < 0 || start.1 < 0 || (start.0 as usize) >= inputs.len() || (start.1 as usize) >= inputs[0].len() || seen.contains(&start) {
        return 0;
    }

    seen.insert(start);

    if inputs[start.0 as usize][start.1 as usize] == 9 {
        return 0;
    }

    basin(inputs, (start.0 + 1, start.1), seen) + basin(inputs, (start.0, start.1 + 1), seen) +
    basin(inputs, (start.0 - 1, start.1), seen) + basin(inputs, (start.0, start.1 - 1), seen) + 1
}


fn part_2() -> usize {
    let inputs = get_inputs();
    let mut seen = HashSet::new();

    all_lows(&inputs)
        .iter()
        .map(|&l| basin(&inputs, l, &mut seen))
        .collect::<BinaryHeap<usize>>()
        .iter()
        .take(3)
        .product::<usize>()
}


fn main() -> Result<()> {
    println!("{}", part_1());
    println!("{}", part_2());
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert!(part_1() == 532);
    }

    
    #[test]
    fn test_part_2() {
        assert!(part_2() == 1_110_780);
    }
}
