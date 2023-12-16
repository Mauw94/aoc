use std::iter::zip;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let (times, distances) = parse_input(input)?;

    Some(simulate(&times, &distances))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (time, distance) = parse_input_p2(input)?;

    Some(simulate(&[time], &[distance]))
}

fn simulate(times: &[f64], distances: &[f64]) -> u32 {
    let mut wins = 1;

    for (time, distance) in zip(times, distances) {
        let mut margin = 0;
        for hold in 0..*time as usize {
            if hold * (*time as usize - hold) > *distance as usize {
                margin += 1;
            }
        }
        wins *= margin;
    }

    wins
}

fn parse_input(input: &str) -> Option<(Vec<f64>, Vec<f64>)> {
    let lines: Vec<&str> = input.lines().collect();
    let mut times: Vec<f64> = Vec::new();
    let mut distance: Vec<f64> = Vec::new();

    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();

        let values: Vec<f64> = words[1..].iter().filter_map(|&x| x.parse().ok()).collect();

        match words[0] {
            "Time:" => times.extend_from_slice(&values),
            "Distance:" => distance.extend_from_slice(&values),
            _ => {}
        }
    }

    Some((times, distance))
}

fn parse_input_p2(input: &str) -> Option<(f64, f64)> {
    let lines: Vec<&str> = input.lines().collect();
    let mut time: f64 = 0.0;
    let mut distance: f64 = 0.0;

    for line in lines {
        let words: Vec<&str> = line.split_whitespace().collect();

        let value = words[1..].join("").parse::<f64>().ok()?;

        match words[0] {
            "Time:" => time = value,
            "Distance:" => distance = value,
            _ => {}
        }
    }

    Some((time, distance))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
