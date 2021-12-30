use aoc::*;


fn get_inputs() -> Vec<i32>{
    include_str!("../../input/day6.txt")
        .lines()
        .map(|s| s.split(',').collect::<Vec<&str>>())
        .flatten()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>()
}


fn calculate(iterations: usize) -> usize {
    let mut fish: [usize; 9] = [0; 9];

    for i in get_inputs() {
        fish[i as usize] += 1;
    }

    for _ in 0..iterations {
        let mut previous = fish[8];
        for i in (1..8).rev() {
            std::mem::swap(&mut fish[i], &mut previous);
        }
        fish[6] += fish[0];
        fish[8] = fish[0];
        fish[0] = previous;
    }

    fish
        .iter()
        .sum()
}


fn part_1() -> usize {
    calculate(80)
}


fn part_2() -> usize {
    calculate(256)
}


fn main() -> Result<()> {
    println!("{:?}", part_1());
    println!("{:?}", part_2());
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert!(part_1() == 390_923);
    }

    #[test]
    fn test_part_2() {
        assert!(part_2() == 1_749_945_484_935);
    }
}
