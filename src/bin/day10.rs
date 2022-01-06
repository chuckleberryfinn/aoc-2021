use aoc::*;
use std::collections::{BinaryHeap, HashMap};

fn get_inputs() -> Vec<&'static str> {
    include_str!("../../input/day10.txt")
        .lines()
        .collect()
}


fn corrupt_score(input: &str) -> usize {
    let symbols = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let symbol_scores = HashMap::from([(')', 3), (']', 57), ('}', 1_197), ('>', 25_137)]);
    let mut stack = Vec::new();

    for c in input.chars() {
        if symbols.contains_key(&c) {
            stack.push(c);
        } else if *symbols.get(&stack.pop().unwrap()).unwrap() != c {
            return *symbol_scores.get(&c).unwrap();
        }
    }
    0
}


fn part_1() -> usize {
    let inputs = get_inputs();

    inputs
        .iter()
        .map(|input| corrupt_score(input))
        .sum::<usize>()
}


fn part_2() -> usize {
    let inputs = get_inputs();
    let symbols = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let mut scores = BinaryHeap::new();

    for input in inputs.iter().filter(|input| corrupt_score(input) == 0) {
        let mut stack = Vec::new();
        for c in input.chars() {
            if symbols.contains_key(&c) {
                stack.push(c);
            } else {
                stack.pop();
            }
        }

        scores.push(
            stack
                .iter()
                .fold(0, |total, v| (total * 5) + symbols.get(v).unwrap())
        );
    }

    let n = scores.len();
    scores.into_sorted_vec()[n/2]
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
        assert!(part_1() == 387_363);
    }

    #[test]
    fn test_part_2() {
        assert!(part_2() == 4_330_777_059);
    }
}