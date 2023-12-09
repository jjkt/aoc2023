use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::ops::Range;
use std::sync::mpsc::channel;

#[derive(Debug)]
struct Map {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u32,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Range<u64>>,
    maps: HashMap<String, Vec<Map>>,
}

impl std::str::FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let destination_range_start = parts.next().unwrap().parse::<u64>()?;
        let source_range_start = parts.next().unwrap().parse::<u64>()?;
        let range_length = parts.next().unwrap().parse::<u32>()?;

        Ok(Map {
            destination_range_start,
            source_range_start,
            range_length,
        })
    }
}

fn parse_input(
    filename: &str,
    map_names: &Vec<&str>,
    seeds_are_ranges: bool,
) -> Result<Almanac, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    let mut almanac = Almanac {
        seeds: Vec::new(),
        maps: HashMap::new(),
    };

    let mut parsing_map = false;
    let mut current_map = String::new();

    for line in contents.lines() {
        if parsing_map {
            // if line is empty (after trimming whitespace), then stop parsing the map.
            if line.trim().is_empty() {
                parsing_map = false;
            } else {
                // if line starts with a number, then parse it into a map.
                if let Ok(map) = line.parse::<Map>() {
                    almanac.maps.get_mut(&current_map).unwrap().push(map);
                }
            }
        } else {
            // if line starts with "seeds:", then parse them into seeds.
            // each seed is a number separated by spaces.
            if line.starts_with("seeds:") {
                if seeds_are_ranges {
                    // the seeds are pair of numbers with start and length, separated by a space.
                    // there can be multiple pairs. parse them into seeds:
                    let seeds: Vec<&str> = line.split_whitespace().skip(1).collect();
                    for pair in seeds.chunks(2) {
                        let start = pair[0].parse::<u64>()?;
                        let length = pair[1].parse::<u64>()?;
                        almanac.seeds.push(start..start + length);
                    }
                } else {
                    for seed in line.split_whitespace().skip(1) {
                        let parsed_seed: u64 = seed.parse::<u64>()?;
                        almanac.seeds.push(parsed_seed..parsed_seed + 1);
                    }
                }
            } else {
                // if the line starts with one of the map names, plus "map:" then start parsing the map.
                for map_name in map_names {
                    if line.starts_with(map_name) {
                        parsing_map = true;
                        almanac.maps.insert(map_name.to_string(), Vec::new());
                        current_map = map_name.to_string();
                    }
                }
            }
        }
    }
    Ok(almanac)
}

fn map_single_value(value: u64, map: &Map) -> Option<u64> {
    if value >= map.source_range_start && value < map.source_range_start + map.range_length as u64 {
        let offset = value - map.source_range_start;
        Some(map.destination_range_start + offset)
    } else {
        None
    }
}

fn map_value(value: u64, maps: &Vec<Map>) -> u64 {
    let mut destination = value;
    for map in maps {
        if let Some(new_destination) = map_single_value(destination, map) {
            destination = new_destination;
            break;
        }
    }
    destination
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_value() {
        let maps = vec![
            Map {
                destination_range_start: 50,
                source_range_start: 98,
                range_length: 2,
            },
            Map {
                destination_range_start: 52,
                source_range_start: 50,
                range_length: 48,
            },
        ];
        assert_eq!(map_value(79, &maps), 81);
        assert_eq!(map_value(14, &maps), 14);
        assert_eq!(map_value(55, &maps), 57);
        assert_eq!(map_value(13, &maps), 13);
    }
}

fn process_file(
    filename: &str,
    map_names: &Vec<&str>,
    seeds_are_ranges: bool,
) -> Result<(), Box<dyn Error>> {
    let almanac = parse_input(filename, &map_names, seeds_are_ranges)?;

    println!("seed count: {:?}", almanac.seeds.len());
    let (sender, receiver) = channel();

    almanac
        .seeds
        .into_par_iter()
        .for_each_with(sender, |sender, seed_range| {
            let mut smallest_destination = u64::MAX;
            let mut counter = 0;
            let total_seeds = seed_range.end - seed_range.start;
            println!(
                "{}: will count {:?} seeds",
                rayon::current_thread_index().unwrap(),
                total_seeds
            );
            for seed in seed_range {
                let mut destination = seed;
                for map_name in map_names {
                    let maps = almanac.maps.get(&map_name.to_string()).unwrap();

                    destination = map_value(destination, maps);
                }
                if destination < smallest_destination {
                    smallest_destination = destination;
                }
                counter += 1;

                if counter % 10000000 == 0 {
                    println!(
                        "{}: {:.2}%",
                        rayon::current_thread_index().unwrap(),
                        (counter as f64 / total_seeds as f64) * 100.0
                    );
                }
            }
            println!(
                "{}: sending {}",
                rayon::current_thread_index().unwrap(),
                smallest_destination
            );
            sender.send(smallest_destination).unwrap();
        });

    let mut destinations: Vec<_> = receiver.iter().collect();
    destinations.sort();

    let minimum_destination = destinations[0];

    println!("{} minimum destination: {}", filename, minimum_destination);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let map_names = vec![
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    // Part 1
    process_file("input.txt", &map_names, false)?;
    process_file("input_full.txt", &map_names, false)?;

    // Part 2
    process_file("input.txt", &map_names, true)?;
    process_file("input_full.txt", &map_names, true)?;

    Ok(())
}
