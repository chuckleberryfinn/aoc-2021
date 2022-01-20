use aoc::*;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

//https://doc.rust-lang.org/std/collections/binary_heap/index.html
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}


impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}


impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn get_inputs() -> Vec<Vec<usize>> {
    include_str!("../../input/day15.txt")
        .lines()
        .map(|line|
            line
                .as_bytes()
                .iter()
                .map(|&weight| (weight as char).to_digit(10).unwrap() as usize)
                .collect()
        )
        .collect()
}


fn shortest_path(cave: &[Vec<usize>]) -> usize {
    let goal = (cave.len() - 1, cave[0].len() - 1);

    let mut heap = BinaryHeap::new();
    let mut dist: Vec<_> =
        (0..cave.len())
            .map(|_|
                (0..cave[0].len())
                    .map(|_| usize::MAX)
                    .collect::<Vec<usize>>()
            )
            .collect::<Vec<Vec<usize>>>();

    dist[0][0] = 0;
    heap.push(State { cost: 0, position: (0, 0) });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal { return cost; }
        if cost > dist[position.0][position.1] { continue; }

        for edge in [(position.0.saturating_sub(1), position.1), (position.0 + 1, position.1), (position.0, position.1.saturating_sub(1)), (position.0, position.1 + 1)] {
            if edge.0 >= cave.len() || edge.1 >= cave[0].len() {
                continue;
            }

            let next = State { cost: cost + cave[edge.0][edge.1], position: (edge.0, edge.1) };
            if next.cost < dist[edge.0][edge.1] {
                heap.push(next);
                dist[next.position.0][next.position.1] = next.cost;
            }
        }
    }
    unreachable!();
}


fn part_1() -> usize {
    let cave = get_inputs();
    shortest_path(&cave)
}


fn part_2() -> usize {
    let cave = get_inputs();

    let full_cave =
        (0..cave.len() * 5)
            .map(|r| {
                let adj_r = (r / cave.len(), r % cave.len());
                (0..cave[0].len() * 5)
                    .map(|c| {
                        let adj_c = (c / cave[0].len(), c % cave[0].len());
                        let mut v = cave[adj_r.1][adj_c.1];
                        v += adj_c.0 + adj_r.0;

                        if v != 9 {
                            v %= 9;
                        }
                        v
                    })
                .collect::<Vec<usize>>()
            })
        .collect::<Vec<Vec<usize>>>();

    shortest_path(&full_cave)
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
        assert!(part_1() == 685);
    }

    #[test]
    fn test_part_2() {
        assert!(part_2() == 2_995);
    }
}
