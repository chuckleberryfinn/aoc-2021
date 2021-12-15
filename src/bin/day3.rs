use aoc::*;


fn main() -> Result<()> {
    println!("Part 1 {}", part_1());
    println!("Part 2 {}", part_2());
    Ok(())
}


fn part_1() -> usize {
    get_result(&get_inputs())
}


fn get_inputs() -> Vec<i32> {
    include_str!("../../input/day3.txt")
        .lines()
        .map(|l| i32::from_str_radix(l, 2).unwrap())
        .collect()
}


fn get_result(inputs: &[i32]) -> usize {
    let g = (0..12)
        .map(|i| most_common(inputs, i))
        .fold(0, |x, c|
            match c {
                1 => (x << 1) + 1,
                _ => x << 1
            }
        );

    g * (g ^ 4095)
}


fn most_common(inputs: &[i32], position: i32) -> i32 {
    let w = inputs
        .iter()
        .fold((0, 0), |(zero, one), input|
            match input & (2048 >> position) {
                0 => (zero + 1, one),
                _ => (zero, one + 1)
            }
        );
        if w.1 >= w.0 {1} else {0}
}


fn part_2() -> i32 {
    get_result_2(&get_inputs())
}


fn get_result_2(inputs: &[i32]) -> i32 {
    get_life_support_value(inputs, false) * get_life_support_value(inputs, true)
}


fn get_life_support_value(inputs: &[i32], invert: bool) -> i32 {
    (0..12)
        .fold(0, |mask, i| {
            let filtered = inputs
                .iter()
                .cloned()
                .filter(|input| (input >> (12-i)) == mask)
                .collect::<Vec<i32>>();

            match filtered.len() {
                1 => filtered[0] >> (12 - (i + 1)),
                _ => {
                    if invert {
                        (mask << 1) | (!most_common(&filtered, i) & 1)
                    } else {
                        (mask << 1) | (most_common(&filtered, i) & 1)
                    }
                }
            }
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert!(part_1() == 841_526);
    }

    
    #[test]
    fn test_part_2() {
        assert!(part_2() == 4_790_390);
    }
}
