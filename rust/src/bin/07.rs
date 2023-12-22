use std::collections::HashMap;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    let letter_map: HashMap<char, char> =
        [('T', 'A'), ('J', 'B'), ('Q', 'C'), ('K', 'D'), ('A', 'E')]
            .iter()
            .cloned()
            .collect();
    let mut plays = parse_plays(input);

    plays.sort_by_key(|play| strength(&play.0, &letter_map));

    let mut total = 0;

    for (rank, (_, bid)) in plays.iter().enumerate() {
        total += (rank + 1) * bid;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let letter_map: HashMap<char, char> =
        [('T', 'A'), ('J', '.'), ('Q', 'C'), ('K', 'D'), ('A', 'E')]
            .iter()
            .cloned()
            .collect();

    let mut plays = parse_plays(input);

    plays.sort_by_key(|play| strength_p2(&play.0, &letter_map));

    let mut total = 0;

    for (rank, (_, bid)) in plays.iter().enumerate() {
        total += (rank + 1) * bid;
    }

    Some(total)
}

fn score(hand: &str) -> usize {
    let counts: Vec<usize> = hand
        .chars()
        .map(|card| hand.chars().filter(|&c| c == card).count())
        .collect();

    if counts.contains(&5) {
        return 6;
    }
    if counts.contains(&4) {
        return 5;
    }
    if counts.contains(&3) {
        if counts.contains(&2) {
            return 4;
        }
        return 3;
    }
    if counts.iter().filter(|&&x| x == 2).count() == 4 {
        return 2;
    }
    if counts.contains(&2) {
        return 1;
    }
    0
}

fn replacements(hand: &str) -> Vec<String> {
    if hand.is_empty() {
        return vec!["".to_string()];
    }

    let first_card = if hand.starts_with('J') {
        "23456789TQKA"
    } else {
        &hand[0..1]
    };

    let mut result = Vec::new();
    for x in first_card.chars() {
        for y in replacements(&hand[1..]) {
            result.push(format!("{}{}", x, y));
        }
    }

    result
}

fn classify(hand: &str) -> usize {
    replacements(hand)
        .iter()
        .map(|x| score(x))
        .max()
        .unwrap_or(0)
}

fn strength(hand: &str, letter_map: &HashMap<char, char>) -> (usize, Vec<char>) {
    let classified = score(hand);
    let mapped_hand: Vec<char> = hand
        .chars()
        .map(|card| *letter_map.get(&card).unwrap_or(&card))
        .collect();

    (classified, mapped_hand)
}

fn strength_p2(hand: &str, letter_map: &HashMap<char, char>) -> (usize, Vec<char>) {
    let classified = classify(hand);
    let mapped_hand: Vec<char> = hand
        .chars()
        .map(|card| *letter_map.get(&card).unwrap_or(&card))
        .collect();

    (classified, mapped_hand)
}

fn parse_plays(input: &str) -> Vec<(String, usize)> {
    let mut plays: Vec<(String, usize)> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let hand = parts[0].to_string();
        let bid: usize = parts[1].parse().unwrap();
        plays.push((hand, bid));
    }

    plays
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
