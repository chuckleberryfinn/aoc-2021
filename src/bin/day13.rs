use aoc::*;

use std::collections::HashSet;
use itertools::Itertools;

fn get_inputs() -> (Vec<(usize, usize)>, Vec<(char, usize)>) {
    let all_lines =
        include_str!("../../input/day13.txt")
            .lines()
            .collect::<Vec<&'static str>>();

    let points =
        all_lines
            .iter()
            .filter(|s| s.chars().contains(&','))
            .map(|s| s.split(',').collect_tuple().unwrap())
            .map(|n: (&str, &str)| (n.0.parse().unwrap(), n.1.parse().unwrap()))
            .collect();
    
    let folds =
        all_lines
            .iter()
            .filter(|s| s.chars().contains(&'='))
            .map(|s| s.split('=').collect_tuple().unwrap())
            .map(|n: (&str, &str)| (n.0.chars().last().unwrap(), n.1.parse().unwrap()))
            .collect();

    (points, folds)
}


fn remap_points(points: &Vec<(usize, usize)>, folds: &[(char, usize)]) -> Vec<(usize, usize)> {
    points
        .iter()
        .map(|p|
            folds
                .into_iter()
                .fold((p.0, p.1), |result, &fold|
                    match fold.0 == 'x' {
                        true => {
                            match result.0 > fold.1 {
                                true => (fold.1 - (result.0 - fold.1), result.1),
                                _ => (result.0, result.1)
                            }
                        },
                        _ => match result.1 > fold.1 {
                            true => (result.0, fold.1 - (result.1 - fold.1)),
                            _ => (result.0, result.1)
                        }
                    }
                )
        )
        .collect::<Vec<(usize, usize)>>()
}


fn part_1() -> usize {
    let (points, folds) = get_inputs();
    remap_points(&points, &folds[0..1])
        .into_iter()
        .collect::<HashSet<(usize, usize)>>()
        .len()
}


fn part_2() -> String {
    let (points, folds) = get_inputs();

    let filtered_points =
        remap_points(&points, &folds)
            .into_iter()
            .collect::<HashSet<(usize, usize)>>();

    let (x_max, y_max) =
        filtered_points
            .iter()
            .copied()
            .reduce(|result, p| (std::cmp::max(p.0, result.0), std::cmp::max(p.1, result.1))).unwrap();

    (0..=y_max)
        .map(|y| 
            (0..=x_max)
                .map(|x|
                    match filtered_points.contains(&(x, y)) {
                        true => 'X',
                        _ => '.'
                    }
                )
            .collect()
        )
        .map(|s: Vec<char>| s.iter().join(" "))
        .join("\n")
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
        assert!(part_1() == 701);
    }

    
    #[test]
    fn test_part_2() {
        assert!(part_2() == ("X X X X . X X X . . X X X X . X . . X . X X X . . X X X X . . . X X . X . . .\n".to_owned() +
                             "X . . . . X . . X . X . . . . X . X . . X . . X . X . . . . . . . X . X . . .\n" +
                             "X X X . . X . . X . X X X . . X X . . . X X X . . X X X . . . . . X . X . . .\n" +
                             "X . . . . X X X . . X . . . . X . X . . X . . X . X . . . . . . . X . X . . .\n" +
                             "X . . . . X . . . . X . . . . X . X . . X . . X . X . . . . X . . X . X . . .\n" +
                             "X . . . . X . . . . X X X X . X . . X . X X X . . X X X X . . X X . . X X X X"));
    }
}