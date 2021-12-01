use aoc::*;


fn get_inputs() -> Vec<i32> {
    input("day1.txt")
        .unwrap()
        .lines()
        .map(|s| s.trim().parse().unwrap())
        .collect::<Vec<i32>>()
}


fn get_result() -> usize {
    get_inputs().windows(2)
        .filter(|w| w[1] > w[0])
        .count()
}


fn get_result_2() -> usize {
    get_inputs().windows(4)
        .filter(|w| w[1..4].iter().sum::<i32>() > w[0..3].iter().sum::<i32>())
        .count()
}


fn main() -> Result<()> {
    println!("Part 1 {}", get_result());
    println!("Part 2 {}", get_result_2());
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert!(get_result() == 1_451);
    }

    
    #[test]
    fn test_part_2() {
        assert!(get_result_2() == 1_395);
    }
}
