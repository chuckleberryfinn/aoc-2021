use aoc::*;


fn get_inputs() -> Vec<(&'static str, usize)> {
    include_str!("../../input/day2.txt")
        .lines()
        .map(|s| s.splitn(2, ' ').collect::<Vec<&str>>())
        .map(|s| (s[0], s[1].parse().unwrap()))
        .collect()
}


fn get_result(directions: &[(&'static str, usize)]) -> (usize, usize, usize) {
    directions
        .iter()
        .fold((0, 0, 0), |(horizontal, depth, aim), d|
            match d.0 {
                "forward" => (horizontal + d.1, depth + aim * d.1, aim),
                "up" => (horizontal, depth, aim - d.1),
                _ => (horizontal, depth, aim + d.1)
            }
        )
}


fn part_1() -> usize {
    let result = get_result(&get_inputs());
    result.0 * result.2
}


fn part_2() -> usize {
    let result = get_result(&get_inputs());
    result.0 * result.1
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
        assert!(part_1() == 1_924_923);
    }

    
    #[test]
    fn test_part_2() {
        assert!(part_2() == 1_982_495_697);
    }
}
