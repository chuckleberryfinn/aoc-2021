use aoc::*;


fn get_inputs() -> Vec<i32> {
    input("day1.txt")
        .unwrap()
        .lines()
        .map(|s| s.trim().parse().unwrap())
        .collect::<Vec<i32>>()
}


fn get_sums(input: &[i32]) -> Vec<i32> {
    input
        .windows(3)
        .into_iter()
        .map(|x| x.iter().sum())
        .collect::<Vec<i32>>()
}


fn get_result(input: &[i32]) -> usize {
    input
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count()
}


fn main() -> Result<()> {
    println!("Part 1 {}", get_result(&get_inputs()));
    println!("Part 2 {}", get_result(&get_sums(&get_inputs())));
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert!(get_result(&get_inputs()) == 1_451);
    }

    
    #[test]
    fn test_part_2() {
        assert!(get_result(&get_sums(&get_inputs())) == 1_395);
    }
}
