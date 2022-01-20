use aoc::*;

use std::collections::HashMap;
use std::str;

use itertools::Itertools;


fn get_inputs() -> (&'static str, HashMap<&'static str, u8>) {
    include_str!("../../input/day14.txt")
        .split_once("\n\n")
        .map(|(pattern, instructions)| (
            pattern,
            instructions
                .lines()
                .map(|line| line.split_once(" -> ").unwrap())
                .map(|l: (&str, &str)| (l.0, *l.1.as_bytes().iter().next().unwrap()))
                .collect()
        ))
        .unwrap()
}


fn calculate(iterations: usize) -> usize {
    let (pattern, instructions) = get_inputs();

    let mut counts =
        instructions
            .keys()
            .map(|&k| (k, 0))
            .collect::<HashMap<&str, usize>>();

    for c in pattern.as_bytes().windows(2) {
        *counts.get_mut(str::from_utf8(c).unwrap()).unwrap() += 1;
    }

    for _ in 0..iterations {
        for (k, v) in counts.clone() {
            *counts.get_mut(k).unwrap() -= v;
            let insert = instructions.get(k).unwrap();
            let (f, l) = k.as_bytes().iter().collect_tuple().unwrap();
            *counts.get_mut(str::from_utf8(&[*f, *insert]).unwrap()).unwrap() += v;
            *counts.get_mut(str::from_utf8(&[*insert, *l]).unwrap()).unwrap() += v;
        }
    }

    let mut final_counts: HashMap<char, usize> = HashMap::new();

    for (k,v) in &counts {
        let (f,l) = k.as_bytes().iter().map(|&c| c as char).collect_tuple().unwrap();
        *final_counts.entry(f).or_insert(0) += v;
        *final_counts.entry(l).or_insert(0) += v;
    }

    let (min, max) =
        final_counts
            .values()
            .map(|v|
                match v % 2 == 0 {
                    true => v / 2,
                    false => (v + 1) / 2
                }
            )
            .minmax()
            .into_option()
            .unwrap();

    max - min
}


fn part_1() -> usize {
    calculate(10)
}


fn part_2() -> usize {
    calculate(40)
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
        assert!(part_1() == 3_118);
    }

    #[test]
    fn test_part_2() {
        assert!(part_2() == 4_332_887_448_171);
    }
}
