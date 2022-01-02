use aoc::*;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};


fn get_inputs() -> Vec<(Vec<&'static str>, Vec<&'static str>)>{
    include_str!("../../input/day8.txt")
        .lines()
        .map(|s| s.split('|').collect_tuple().unwrap())
        .map(|n: (&str, &str)| (n.0.split_whitespace().collect(), n.1.split_whitespace().collect()))
        .collect()
}


fn part_1() -> usize {
    get_inputs()
        .into_iter()
        .flat_map(|s| s.1)
        .filter(|s| [2, 3, 4, 7].contains(&s.len()))
        .count()
}


fn part_2() -> i32 {
    get_inputs()
        .iter()
        .fold(0, |total, input| {
            let all_hashes = 
                input.0
                    .iter()
                    .map(|d| HashSet::from_iter(d.chars()))
                    .collect::<Vec<HashSet<char>>>();

            let digits =
                [(2, 1), (3, 7), (4, 4), (7, 8)]
                    .iter()
                    .map(|s|
                        (s.1, all_hashes
                            .iter()
                            .find(|&d| d.len() == s.0)
                            .unwrap())
                    )
                    .collect::<HashMap<i32, &HashSet<char>>>();

            let all_digits =
                all_hashes
                    .iter()
                    .filter(|h| [5, 6].contains(&h.len()))
                    .map(|h|
                        match h.len() {
                            5 => {
                                if h.union(digits.get(&7).unwrap()).collect::<HashSet<&char>>().len() == 5 {
                                    (3, h)
                                } else if h.union(digits.get(&4).unwrap()).collect::<HashSet<&char>>().len() == 7 {
                                    (2, h)
                                } else {
                                    (5, h)
                                }
                            },
                            _ => {
                                if h.union(digits.get(&4).unwrap()).collect::<HashSet<&char>>().len() == 6 {
                                    (9, h)
                                } else if h.union(digits.get(&1).unwrap()).collect::<HashSet<&char>>().len() == 7 {
                                    (6, h)
                                } else {
                                    (0, h)
                                }
                            }
                        }
                    )
                    .collect::<HashMap<i32, &HashSet<char>>>()
                    .into_iter()
                    .chain(digits)
                    .collect::<HashMap<i32, &HashSet<char>>>();

            total + input.1
                .iter()
                .fold(0, |acc, d|
                    acc * 10 + all_digits
                        .iter()
                        .find_map(|(key, val) | if **val == HashSet::from_iter(d.chars()) { Some(key) } else { None })
                        .unwrap()
                )
    })
}


fn main() -> Result<()>{
    println!("Part 1 {}", part_1());
    println!("Part 2 {}", part_2());
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert!(part_1() == 525);
    }

    #[test]
    fn test_part_2() {
        assert!(part_2() == 1_083_859);
    }
}
