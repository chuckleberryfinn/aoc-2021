use aoc::*;

use std::collections::HashMap;
use itertools::Itertools;

type Point = (i32, i32);
type PointPair = (Point, Point);

fn get_inputs() -> Vec<PointPair>{
    include_str!("../../input/day5.txt")
        .lines()
        .map(|s| {
                let mut points = s
                    .split(" -> ")
                    .map(|s: &str| s
                            .split(',')
                            .map(|n| n.parse().unwrap())
                            .collect_tuple()
                            .unwrap()
                    ).collect::<Vec<Point>>();
                points.sort_by_cached_key(|k| (k.0, k.1));
                (points[0], points[1])
        })
        .collect()
}


fn straight_lines(point_pairs: &[PointPair]) -> usize {
    all_lines(&point_pairs
        .iter()
        .cloned()
        .filter(|pp| (pp.0.0 == pp.1.0) | (pp.0.1 == pp.1.1))
        .collect::<Vec<PointPair>>())
}


fn all_lines(point_pairs: &[PointPair]) -> usize {
    let mut point_counts: HashMap<Point, i32> = HashMap::new();

    for point in point_pairs.iter().cloned() {
        match point.0.0 {
            _ if point.0.0 < point.1.0 => {
                for (i, j) in (point.0.0..=point.1.0).enumerate() {
                    *point_counts.entry(
                        match point.0.1 {
                            p if p == point.1.1 => (j, p),
                            p if p > point.1.1 => (j, p - i as i32),
                            _ => (j, point.0.1 + i as i32)
                        }
                    ).or_insert(0) += 1;
                }
            },
            _ => {
                for (i, j) in (point.0.1..=point.1.1).enumerate() {
                    *point_counts.entry(
                        match point.0.0 {
                            p if p == point.1.0 => (p, j),
                            p if p < point.1.0 => (p + i as i32, j),
                            _ => (point.0.0 - i as i32, j)
                        }
                    ).or_insert(0) += 1;
                }
            }
        }
    }

    point_counts
        .into_iter()
        .filter(|(_, v)| *v > 1)
        .count()
}


fn part_1() -> usize {
    straight_lines(&get_inputs())
}


fn part_2() -> usize {
    all_lines(&get_inputs())
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
        assert!(part_1() == 6_007);
    }

    #[test]
    fn test_part_2() {
        assert!(part_2() == 19_349);
    }
}