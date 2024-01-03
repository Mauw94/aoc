advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let lists = parse_lists(input);
    let histories = calculate_histories(lists, true);

    Some(histories.iter().sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let lists = parse_lists(input);
    let histories = calculate_histories(lists, false);

    Some(histories.iter().sum())
}

fn parse_lists(input: &str) -> Vec<Vec<i32>> {
    let mut n: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let numbers: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        n.push(numbers);
    }

    n
}

fn calculate_histories(lists: Vec<Vec<i32>>, part1: bool) -> Vec<i32> {
    let mut histories: Vec<i32> = Vec::new();
    for list in lists {
        let history: Vec<i32>;
        let mut previous: Vec<i32> = Vec::new();
        let extrapolated: i32;

        if part1 {
            previous.push(list.last().unwrap().clone());
            history = subtract_until_zero(list, previous);
            histories.push(history.iter().sum::<i32>());
        } else {
            previous.push(list.first().unwrap().clone());
            history = subtract_until_zero_reversed(list, previous);
            extrapolated = calc_extrapolated_value(&history);
            histories.push(extrapolated);
        }
    }

    histories
}

fn calc_extrapolated_value(history: &Vec<i32>) -> i32 {
    let mut previous: i32 = 0;

    for (i, _) in history.iter().enumerate().rev() {
        previous = history[i] - previous;
    }

    previous
}

fn subtract_until_zero(list: Vec<i32>, mut previous: Vec<i32>) -> Vec<i32> {
    let mut converted: Vec<i32> = Vec::new();

    for i in 0..list.len() {
        if i < list.len() - 1 {
            let x = list[i + 1] - list[i];
            converted.push(x);
        }
    }

    let zero_summed: i32 = converted.iter().sum();
    if zero_summed == 0 {
        return previous;
    }

    previous.push(converted.last().unwrap().clone());

    return subtract_until_zero(converted, previous);
}

fn subtract_until_zero_reversed(list: Vec<i32>, mut previous: Vec<i32>) -> Vec<i32> {
    let mut converted: Vec<i32> = Vec::new();

    for (i, _) in list.iter().enumerate().rev() {
        if i > 0 {
            let x = list[i] - list[i - 1];
            converted.insert(0, x);
        }
    }

    let zero_summed: i32 = converted.iter().sum();
    if zero_summed == 0 {
        return previous;
    }

    previous.push(converted.first().unwrap().clone());

    return subtract_until_zero_reversed(converted, previous);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
