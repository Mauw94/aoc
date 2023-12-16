use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(Clone)]
struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil: Vec<Mapping>,
    soil_to_fert: Vec<Mapping>,
    fert_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temp: Vec<Mapping>,
    temp_to_humid: Vec<Mapping>,
    humid_to_loc: Vec<Mapping>,
}

impl Almanac {
    fn find_locations(&self) -> Vec<usize> {
        let mut locations: Vec<usize> = Vec::new();

        for seed in &self.seeds {
            let soil_nr = self.find_next_location(&self.seed_to_soil, seed);
            let fertilizer = self.find_next_location(&self.soil_to_fert, &soil_nr);
            let water = self.find_next_location(&self.fert_to_water, &fertilizer);
            let light = self.find_next_location(&self.water_to_light, &water);
            let temperature = self.find_next_location(&self.light_to_temp, &light);
            let humidity = self.find_next_location(&self.temp_to_humid, &temperature);
            let location = self.find_next_location(&self.humid_to_loc, &humidity);
            locations.push(location);
        }

        locations
    }

    fn find_next_location(&self, mapping: &Vec<Mapping>, seed_nr: &usize) -> usize {
        let mut location = *seed_nr;
        for m in mapping {
            let max_range = m.source + m.range - 1;
            if seed_nr >= &m.source && seed_nr <= &max_range {
                let diff = max_range - seed_nr;
                location = m.destination + m.range - 1 - diff;
            }
        }

        location
    }

    fn find_lowest_location(&self, seed_nr: &usize) -> usize {
        let soil_nr = self.find_next_location(&self.seed_to_soil, seed_nr);
        let fertilizer = self.find_next_location(&self.soil_to_fert, &soil_nr);
        let water = self.find_next_location(&self.fert_to_water, &fertilizer);
        let light = self.find_next_location(&self.water_to_light, &water);
        let temperature = self.find_next_location(&self.light_to_temp, &light);
        let humidity = self.find_next_location(&self.temp_to_humid, &temperature);

        self.find_next_location(&self.humid_to_loc, &humidity)
    }

    fn get_lowest_destination(&self) -> usize {
        let mut lowest = usize::MAX;

        for (idx, _) in self.seeds.iter().enumerate().step_by(2) {
            let range = self.seeds[idx + 1];
            for j in 0..range {
                let s = self.seeds[idx] + j;
                let location = self.find_lowest_location(&s);
                lowest = std::cmp::min(location, lowest);
            }
        }

        lowest
    }
}

#[derive(Clone)]
struct Mapping {
    destination: usize,
    source: usize,
    range: usize,
}

pub fn part_one(input: &str) -> Option<usize> {
    let almanac = parse_almanac(input)?;
    let locations = almanac.find_locations();

    Some(*locations.iter().min()?)
}

pub fn part_two(input: &str) -> Option<usize> {
    let almanac = parse_almanac(input)?;

    Some(almanac.get_lowest_destination())
}

fn parse_almanac(input: &str) -> Option<Almanac> {
    let seeds = input
        .lines()
        .next()?
        .split_once(':')?
        .1
        .split_ascii_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect_vec();

    let maps = input
        .split("\n\n")
        .skip(1)
        .filter_map(parse_map)
        .collect_vec();

    Some(Almanac {
        seeds,
        seed_to_soil: maps[0].clone(),
        soil_to_fert: maps[1].clone(),
        fert_to_water: maps[2].clone(),
        water_to_light: maps[3].clone(),
        light_to_temp: maps[4].clone(),
        temp_to_humid: maps[5].clone(),
        humid_to_loc: maps[6].clone(),
    })
}

fn parse_map(chunk: &str) -> Option<Vec<Mapping>> {
    let ranges: Vec<Mapping> = chunk
        .lines()
        .filter_map(|l| {
            if l.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                let mut splits = l.split(' ').filter_map(|x| x.parse::<usize>().ok());
                Some(Mapping {
                    destination: splits.next()?,
                    source: splits.next()?,
                    range: splits.next()?,
                })
            } else {
                None
            }
        })
        .collect_vec();

    if ranges.is_empty() {
        None
    } else {
        Some(ranges)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
