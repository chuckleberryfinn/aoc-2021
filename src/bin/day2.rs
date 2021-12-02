use aoc::*;


fn parse_line(p: &str) -> (String, usize) {
    let x: Vec<&str> = p.split(' ').collect();
    (
        x[0].to_string(),
        x[1].parse::<usize>().unwrap()
    )
}


fn get_inputs() -> Vec<(String, usize)> {
    input("day2.txt")
        .unwrap()
        .lines()
        .map(|s| parse_line(s))
        .collect::<Vec<(String, usize)>>()
}


fn get_result(directions: &Vec<(String, usize)>) -> (usize, usize, usize) {
    directions
        .iter()
        .fold((0, 0, 0), |(horizontal, depth, aim), d|
            match d.0.as_str() {
                "forward" => (horizontal + d.1, depth + aim * d.1, aim),
                "up" => (horizontal, depth, aim - d.1),
                "down" => (horizontal, depth, aim + d.1),
                _ => panic!("Unexpected input")
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
