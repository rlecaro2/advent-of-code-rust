use itertools::Itertools;
use rangemap::RangeMap;
use regex::Regex;

advent_of_code::solution!(5);

fn get_next_value(map: &RangeMap<u64, u64>, val: u64) -> u64 {
    if map.contains_key(&val) {
        let (source, dest_start) = map.get_key_value(&val).unwrap();
        dest_start + (val - source.start)
    } else {
        val
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let section_re = Regex::new(r"\w+-to-\w+").unwrap();
    let map_re = Regex::new(r"(?<destination>\d+) (?<source>\d+) (?<size>\d+)").unwrap();
    let number_re = Regex::new(r"\d+").unwrap();

    let mut seeds: Vec<u64> = vec![];

    // lol
    // map a range to the start of the next range
    let mut seed_to_soil: RangeMap<u64, u64> = RangeMap::new();
    let mut soil_to_fertilizer: RangeMap<u64, u64> = RangeMap::new();
    let mut fertilizer_to_water: RangeMap<u64, u64> = RangeMap::new();
    let mut water_to_light: RangeMap<u64, u64> = RangeMap::new();
    let mut light_to_temperature: RangeMap<u64, u64> = RangeMap::new();
    let mut temperature_to_humidity: RangeMap<u64, u64> = RangeMap::new();
    let mut humidity_to_location: RangeMap<u64, u64> = RangeMap::new();

    let mut current_range = &mut seed_to_soil;

    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            seeds = number_re
                .find_iter(line)
                .map(|n| n.as_str().parse::<u64>().unwrap())
                .collect();
        }

        if section_re.is_match(line) {
            current_range = match section_re.find(line).unwrap().as_str() {
                "seed-to-soil" => &mut seed_to_soil,
                "soil-to-fertilizer" => &mut soil_to_fertilizer,
                "fertilizer-to-water" => &mut fertilizer_to_water,
                "water-to-light" => &mut water_to_light,
                "light-to-temperature" => &mut light_to_temperature,
                "temperature-to-humidity" => &mut temperature_to_humidity,
                "humidity-to-location" => &mut humidity_to_location,
                _ => current_range,
            }
        }

        for caps in map_re.captures_iter(line) {
            let source = caps["source"].parse::<u64>().unwrap();
            let destination = caps["destination"].parse::<u64>().unwrap();
            let size = caps["size"].parse::<u64>().unwrap();

            let source_range = source..source + size;
            current_range.insert(source_range, destination);
        }
    }

    let min = seeds
        .iter()
        .map(|s| {
            let mut val = *s;
            val = get_next_value(&seed_to_soil, val);
            val = get_next_value(&soil_to_fertilizer, val);
            val = get_next_value(&fertilizer_to_water, val);
            val = get_next_value(&water_to_light, val);
            val = get_next_value(&light_to_temperature, val);
            val = get_next_value(&temperature_to_humidity, val);
            val = get_next_value(&humidity_to_location, val);
            val
        })
        .min()
        .unwrap_or(0);

    return Some(min);
}

pub fn part_two(input: &str) -> Option<u64> {
    let section_re = Regex::new(r"\w+-to-\w+").unwrap();
    let map_re = Regex::new(r"(?<destination>\d+) (?<source>\d+) (?<size>\d+)").unwrap();
    let number_re = Regex::new(r"\d+").unwrap();

    let mut seeds: Vec<(u64, u64)> = vec![];

    // map a range to the start of the next range
    let mut seed_to_soil: RangeMap<u64, u64> = RangeMap::new();
    let mut soil_to_fertilizer: RangeMap<u64, u64> = RangeMap::new();
    let mut fertilizer_to_water: RangeMap<u64, u64> = RangeMap::new();
    let mut water_to_light: RangeMap<u64, u64> = RangeMap::new();
    let mut light_to_temperature: RangeMap<u64, u64> = RangeMap::new();
    let mut temperature_to_humidity: RangeMap<u64, u64> = RangeMap::new();
    let mut humidity_to_location: RangeMap<u64, u64> = RangeMap::new();

    let mut current_range = &mut seed_to_soil;

    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            seeds = number_re
                .find_iter(line)
                .tuples()
                .map(|(n, m)| {
                    let start = n.as_str().parse::<u64>().unwrap();
                    let size = m.as_str().parse::<u64>().unwrap();
                    (start, size)
                })
                .collect();
        }

        if section_re.is_match(line) {
            current_range = match section_re.find(line).unwrap().as_str() {
                "seed-to-soil" => &mut seed_to_soil,
                "soil-to-fertilizer" => &mut soil_to_fertilizer,
                "fertilizer-to-water" => &mut fertilizer_to_water,
                "water-to-light" => &mut water_to_light,
                "light-to-temperature" => &mut light_to_temperature,
                "temperature-to-humidity" => &mut temperature_to_humidity,
                "humidity-to-location" => &mut humidity_to_location,
                _ => current_range,
            }
        }

        for caps in map_re.captures_iter(line) {
            let source = caps["source"].parse::<u64>().unwrap();
            let destination = caps["destination"].parse::<u64>().unwrap();
            let size = caps["size"].parse::<u64>().unwrap();

            let source_range = source..source + size;
            current_range.insert(source_range, destination);
        }
    }

    let min = seeds
        .iter()
        .map(|(start, size)| {
            // let the computer do the work
            // this takes 260s
            (*start..*start + *size)
                .map(|n| {
                    let mut val = n;
                    val = get_next_value(&seed_to_soil, val);
                    val = get_next_value(&soil_to_fertilizer, val);
                    val = get_next_value(&fertilizer_to_water, val);
                    val = get_next_value(&water_to_light, val);
                    val = get_next_value(&light_to_temperature, val);
                    val = get_next_value(&temperature_to_humidity, val);
                    val = get_next_value(&humidity_to_location, val);
                    val
                })
                .min()
                .unwrap_or(0)
        })
        .min()
        .unwrap_or(0);

    return Some(min);
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
        assert_eq!(result, Some(46));
    }
}
