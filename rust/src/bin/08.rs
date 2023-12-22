use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<i32> {
    let instructions = parse_instructions(input);
    let node_map = parse_node_map(input);

    solve("AAA", |c| c == "ZZZ", &instructions, &node_map)
}

pub fn part_two(input: &str) -> Option<usize> {
    let instructions = parse_instructions(input);
    let node_map = parse_node_map(input);

    let start_positions: Vec<&String> = node_map
        .keys()
        .into_iter()
        .filter(|x| x.ends_with('A'))
        .collect();

    let mut total_steps: Vec<usize> = Vec::new();

    for start_pos in start_positions.iter() {
        let steps = solve(start_pos, |c| c.ends_with('Z'), &instructions, &node_map)?;
        total_steps.push(steps as usize);
    }

    Some(least_common_multiple(total_steps))
}

fn least_common_multiple(mut numbers: Vec<usize>) -> usize {
    match (numbers.pop(), numbers.pop()) {
        (Some(a), Some(b)) => {
            let factos_a = prime_factors(a);
            let factos_b = prime_factors(b);
            let gcf = factos_a.intersection(&factos_b).max().unwrap_or(&1);
            numbers.push(a * b / gcf);
            least_common_multiple(numbers)
        }
        (Some(a), None) => a,
        _ => panic!("Can't happen."),
    }
}

fn prime_factors(num: usize) -> HashSet<usize> {
    let mut current = num;
    let mut factors = HashSet::new();
    for i in 2.. {
        while current % i == 0 {
            current /= i;
            factors.insert(i);
        }
        if current == 1 {
            break;
        }
    }
    factors
}

fn solve(
    start: &str,
    is_end: fn(&str) -> bool,
    instructions: &Vec<char>,
    node_map: &HashMap<String, (String, String)>,
) -> Option<i32> {
    let mut z_found = false;
    let mut map_pos = start;
    let mut steps = 0;
    let mut index = 0;

    while !z_found {
        let instruction = instructions.get(index)?;
        steps += 1;
        index += 1;
        if index >= instructions.len() {
            index = 0;
        }
        let values = node_map.get(map_pos)?;

        if *instruction == 'L' {
            map_pos = &values.0;
        } else if *instruction == 'R' {
            map_pos = &values.1;
        }

        if is_end(map_pos) {
            z_found = true;
        }
    }

    Some(steps)
}

fn parse_instructions(input: &str) -> Vec<char> {
    let mut lines = input.lines();
    let first_line = lines.next().expect("Input is empty").trim();
    first_line.chars().filter(|&c| c.is_alphabetic()).collect()
}

fn parse_node_map(input: &str) -> HashMap<String, (String, String)> {
    let mut node_map: HashMap<String, (String, String)> = HashMap::new();

    for line in input.lines().skip(2) {
        let parts: Vec<&str> = line.split('=').collect();
        let key = parts[0].trim().to_string();
        let values = (
            parts[1].trim().chars().skip(1).take(3).collect(),
            parts[1].trim().chars().skip(6).take(3).collect(),
        );

        node_map.insert(key, values);
    }

    node_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
