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


fn get_result(directions: &Vec<(String, usize)>) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;

    for d in directions {
        match d.0.as_str() {
            "forward" => horizontal += d.1,
            "up" => depth -= d.1,
            "down" => depth += d.1,
            _ => panic!("Unexpected input")
        }
    }

    horizontal * depth
}


fn get_result_2(directions: &Vec<(String, usize)>) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for d in directions {
        match d.0.as_str() {
            "forward" => {
                horizontal += d.1;
                depth += aim * d.1;
            }
            "up" => aim -= d.1,
            "down" => aim += d.1,
            _ => panic!("Unexpected input")
        }
    }

    horizontal * depth
}


fn main() -> Result<()> {
    println!("Part 1 {:?}", get_result(&get_inputs()));
    println!("Part 2 {:?}", get_result_2(&get_inputs()));
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert!(get_result(&get_inputs()) == 1_924_923);
    }

    
    #[test]
    fn test_part_2() {
        assert!(get_result_2(&get_inputs()) == 1_982_495_697);
    }
}
