use std::collections::HashMap;

advent_of_code::solution!(4);

#[derive(Debug, Clone)]
struct Card {
    idx: usize,
    left: Vec<usize>,
    right: Vec<usize>,
}

impl Card {
    pub fn product_winning_numbers(&self) -> usize {
        let mut product: usize = 1;
        let w = self.count_winning_numbers();
        if w == 0 {
            return 0;
        }

        for _ in 1..w {
            product *= 2;
        }

        product
    }

    pub fn count_winning_numbers(&self) -> usize {
        let mut count: usize = 0;
        for n in self.left.iter() {
            if self.right.contains(n) {
                count += 1;
            }
        }
        count
    }
}

fn parse_card(line: &str, idx: usize) -> Option<Card> {
    let parts: Vec<&str> = line.split('|').collect();

    let left: Vec<usize> = parts[0]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let right: Vec<usize> = parts[1]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    Some(Card { idx, left, right })
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut cards: Vec<Card> = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        let card = parse_card(line, idx + 1)?;
        cards.push(card);
    }

    Some(
        cards
            .iter()
            .map(|c| c.product_winning_numbers())
            .sum::<usize>(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut cards_dict: HashMap<usize, usize> = HashMap::new();
    let mut cards: Vec<Card> = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        cards_dict.insert(idx + 1, 1);
        cards.push(parse_card(line, idx + 1)?);
    }

    for card in cards.iter() {
        let winning_numbers = card.count_winning_numbers();
        for n in 1..winning_numbers + 1 {
            let id_of_card_to_increase = card.idx + n;
            let increase_amount = get_card_by_key(&cards_dict, card.idx).map(|&i| i)?;
            increase(&mut cards_dict, &increase_amount, &id_of_card_to_increase);
        }
    }

    Some(cards_dict.values().sum())
}

fn get_card_by_key(dict: &HashMap<usize, usize>, key: usize) -> Option<&usize> {
    dict.get(&key)
}

fn increase(dict: &mut HashMap<usize, usize>, amount: &usize, key: &usize) {
    let entry = dict.entry(*key);
    entry.and_modify(|v| *v += amount).or_insert(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
