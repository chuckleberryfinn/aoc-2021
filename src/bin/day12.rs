use aoc::*;

use std::collections::HashMap;
use itertools::Itertools;

fn get_inputs() -> Vec<(&'static str, &'static str)> {
    include_str!("../../input/day12.txt")
        .lines()
        .map(|s| s.split('-').collect_tuple().unwrap())
        .collect()
}


fn make_graph() -> HashMap::<&'static str, Vec<&'static str>> {
    let inputs = get_inputs();
    let mut graph = HashMap::new();
    
    for i in inputs {
        graph.entry(i.0).or_insert_with(Vec::new);
        graph.get_mut(&i.0).unwrap().push(i.1);

        if ![i.0, i.1].contains(&"start") {
            graph.entry(i.1).or_insert_with(Vec::new);
            graph.get_mut(&i.1).unwrap().push(i.0);
        }
    }
    graph
}


fn find_paths(start: &str, graph: &HashMap<&str, Vec<&str>>, mut seen: HashMap<String, usize>, limit: usize) -> usize {
    let s = start.to_string();
    if start == "end" {
        return 1;
    }

    if seen.values().any(|&v| v >= limit) && seen.contains_key(&s) {
        return 0;
    }

    if start.chars().all(|c| c.is_lowercase()) {
        *seen.entry(s).or_insert(0) += 1;
    }

    graph.get(&start).unwrap()
        .iter()
        .map(|&v| find_paths(v, graph, seen.clone(), limit))
        .sum()
}


fn part_1() -> usize {
    find_paths("start", &make_graph(), HashMap::new(), 1)
}


fn part_2() -> usize {
    find_paths("start", &make_graph(), HashMap::new(), 2)
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
        assert!(part_1() == 5_076);
    }

    #[test]
    fn test_part_2() {
        assert!(part_2() == 145_643);
    }
}
