use aoc::*;
use std::collections::HashSet;

fn get_inputs() -> Vec<Vec<u32>> {
    include_str!("../../input/day11.txt")
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>()
}


fn update_octopus(inputs: &mut Vec<Vec<u32>>, x: i32, y: i32, flashers: &mut HashSet<(i32, i32)>) -> usize {
    if x < 0 || y < 0 || (x as usize) >= inputs.len() || (y as usize) >= inputs[0].len() || flashers.contains(&(x, y)) {
        return 0;
    }

    inputs[x as usize][y as usize] += 1;

    if inputs[x as usize][y as usize] != 10 {
        return 0;
    }

    inputs[x as usize][y as usize] = 0;
    flashers.insert((x, y));

    [(x, y - 1), (x, y + 1), (x + 1, y), (x - 1, y), (x - 1, y - 1), (x - 1, y + 1), (x + 1, y + 1), (x + 1, y - 1)]
        .iter()
        .map(|p| update_octopus(inputs, p.0, p.1, flashers))
        .sum::<usize>() + 1
}


fn update_energy(inputs: &mut Vec<Vec<u32>>) -> (usize, usize) {
    let mut flashers = HashSet::new();

    let iterations =
        (0..inputs.len())
            .fold(0, |counter, x|
                counter + (0..inputs[0].len())
                    .fold(0, |acc, y|
                        acc + update_octopus(inputs, x as i32, y as i32, &mut flashers)
                    )
            );

    (iterations, flashers.len())
}


fn part_1() -> usize {
    let mut inputs = get_inputs();
    (0..100).fold(0, |acc, _| acc + update_energy(&mut inputs).0)
}


fn part_2() -> usize {
    let mut inputs = get_inputs();
    (0..).find(|_| update_energy(&mut inputs).1 == 100).unwrap() + 1
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
        assert!(part_1() == 1_743);
    }

    #[test]
    fn test_part_2() {
        assert!(part_2() == 364);
    }
}
