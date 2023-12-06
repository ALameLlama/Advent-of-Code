use std::collections::HashMap;
use std::ops::Range;

fn main() {
    let input = include_str!("./input1.txt");

    let output: u64 = aoc_process(input).unwrap_or_default();

    println!("{output}");
}

fn aoc_process(input: &str) -> Result<u64, &'static str> {
    let mut parsed = input.split("\n\n").collect::<Vec<&str>>().into_iter();

    let (_, seeds) = parsed.next().unwrap().split_once(": ").unwrap();

    let seeds: Vec<u64> = seeds
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let seed_to_soil_map = parse_range_set(parsed.next().unwrap());
    let soil_to_fertilizer_map = parse_range_set(parsed.next().unwrap());
    let fertilizer_to_water_map = parse_range_set(parsed.next().unwrap());
    let water_to_light_map = parse_range_set(parsed.next().unwrap());
    let light_to_temp_map = parse_range_set(parsed.next().unwrap());
    let temp_to_humidity_map = parse_range_set(parsed.next().unwrap());
    let humidity_to_location_map = parse_range_set(parsed.next().unwrap());

    let locations = seeds.iter().map(|seed| {
        let soil = get_map_value(*seed, &seed_to_soil_map);
        let fertilizer = get_map_value(soil, &soil_to_fertilizer_map);
        let water = get_map_value(fertilizer, &fertilizer_to_water_map);
        let light = get_map_value(water, &water_to_light_map);
        let temp = get_map_value(light, &light_to_temp_map);
        let humidity = get_map_value(temp, &temp_to_humidity_map);
        let location = get_map_value(humidity, &humidity_to_location_map);

        location
    });

    return Ok(locations.min().unwrap());
}

fn get_map_value(input: u64, map: &HashMap<u64, Range<u64>>) -> u64 {
    if let Some((dst, src_range)) = map.iter().find(|(_, range)| range.contains(&input)) {
        return dst + (input - src_range.start);
    }

    input
}

fn parse_range_set(data: &str) -> HashMap<u64, Range<u64>> {
    data.lines()
        .skip(1)
        .map(|ln| {
            let parsed: Vec<u64> = ln.split_whitespace().map(|x| x.parse().unwrap()).collect();

            (parsed[0], parsed[1]..(parsed[1] + parsed[2]))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let output: u64 = aoc_process(input).unwrap_or_default();

        assert_eq!(output, 35);
    }
}