use aoc::*;


fn get_inputs() -> Vec<i32> {
    include_str!("../../input/day7.txt")
        .lines()
        .map(|s| s.split(',').collect::<Vec<&str>>())
        .flatten()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>()
}


fn part_1() -> i32 {
    let inputs = get_inputs();
    let maximum = inputs.iter().max().unwrap();

    (0..*maximum)
        .map(|i|
            inputs
                .iter()
                .map(|input| (i - input).abs())
                .sum::<i32>()
        )
        .min()
        .unwrap()
}


fn part_2() -> i32 {
    let inputs = get_inputs();
    let maximum = inputs.iter().max().unwrap();

    (0..*maximum)
        .map(|i|
            inputs
                .iter()
                .map(|input|
                    (0..=((i - input).abs())).sum::<i32>()
                )
                .sum::<i32>()
        )
        .min()
        .unwrap()
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
        assert!(part_1() == 352_997);
    }

    #[test]
    fn test_part_2() {
        assert!(part_2() == 101_571_302);
    }
}
