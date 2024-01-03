advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let lists = parse_lists(input);
    let histories = calculate_histories(lists);

    Some(histories.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_lists(input: &str) -> Vec<Vec<i32>> {
    let mut n: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let numbers: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        n.push(numbers);
    }

    n
}

fn calculate_histories(lists: Vec<Vec<i32>>) -> Vec<i32> {
    let mut histories: Vec<i32> = Vec::new();
    for list in lists {
        let mut previous: Vec<i32> = Vec::new();
        previous.push(list.last().unwrap().clone());
        let history: Vec<i32> = subtract_until_zero(list, previous);
        histories.push(history.iter().sum::<i32>());
    }

    histories
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
        assert_eq!(result, None);
    }
}
