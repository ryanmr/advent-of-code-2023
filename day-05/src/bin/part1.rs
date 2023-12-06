use std::collections::HashMap;

fn main() {
    let input = include_str!("input1.txt");

    let almanac = parse_almanac(input);

    let result = almanac
        .seeds
        .clone()
        .into_iter()
        .map(|seed| find_trace_seed_to_location(&almanac, seed))
        .collect::<Vec<u64>>();

    println!("result = {:#?}", result);

    let minimum = result.into_iter().min().unwrap();

    println!("answer = {}", minimum);
    // 3374647
}

#[derive(Debug)]
struct Map {
    // 50 98 2
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,

    maps: HashMap<String, Vec<Map>>,
}

fn parse_almanac(input: &str) -> Almanac {
    let lines = input.split("\n").map(|f| f.trim()).collect::<Vec<&str>>();
    let seeds_line = lines.first().unwrap().replace("seeds:", "");
    let seeds = seeds_line
        .trim()
        .split(" ")
        .map(|f| f.trim())
        .map(|f| f.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut hash_map: HashMap<String, Vec<Map>> = HashMap::new();

    let mut i = 0;
    loop {
        let line = lines[i];
        if line.contains("map") {
            let name = line.replace("map:", "").trim().to_string();
            let mut map_list: Vec<Map> = vec![];

            loop {
                i = i + 1;
                let forward_line_opt = lines.get(i);

                if forward_line_opt.is_none() {
                    break;
                }

                let forward_line = forward_line_opt.unwrap();

                if forward_line.trim().is_empty() {
                    break;
                }

                println!("forward line: {}", forward_line);

                let parts = forward_line
                    .split(" ")
                    .map(|f| f.trim())
                    .map(|f| f.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();

                let map = Map {
                    destination_range_start: parts[0],
                    source_range_start: parts[1],
                    range_length: parts[2],
                };

                map_list.push(map);
            }

            hash_map.insert(name, map_list);
        }

        i = i + 1;

        if i > lines.len() {
            break;
        }
    }

    let almanac = Almanac {
        seeds,
        maps: hash_map,
    };

    return almanac;
}

fn find_trace_seed_to_location(almanac: &Almanac, seed: u64) -> u64 {
    let order = vec![
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    // seed to soil
    // source to destination

    let mut val = seed;
    for ord in order.into_iter() {
        let maps = almanac.maps.get(ord).unwrap();
        for map in maps.into_iter() {
            if val >= map.source_range_start && val <= map.source_range_start + map.range_length {
                println!(
                    "{}: {} >= {} && {} <= {}",
                    ord,
                    val,
                    map.source_range_start,
                    val,
                    map.source_range_start + map.range_length
                );
                let offset = val - map.source_range_start;
                let dest = map.destination_range_start + offset;

                println!("{}: mapped source {} -> destination {}", ord, val, dest);
                val = dest;
                break; // stop trying maps from the same category
                       // if it is already found
            } else {
                println!("{}: mapped source {} -> destination {}", ord, val, val);
            }
        }
    }

    println!("final val = {}", val);

    return val;
}

#[cfg(test)]
mod tests {
    use crate::{find_trace_seed_to_location, parse_almanac};

    #[test]
    fn it_blends() {
        let input = "
        seeds: 79 14 55 13

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
        56 93 4
        "
        .trim();

        let almanac = parse_almanac(input);

        let seed_79_to_location = find_trace_seed_to_location(&almanac, 79);
        let seed_14_to_location = find_trace_seed_to_location(&almanac, 14);
        let seed_55_to_location = find_trace_seed_to_location(&almanac, 55);
        let seed_13_to_location = find_trace_seed_to_location(&almanac, 13);

        assert_eq!(seed_79_to_location, 82);
        assert_eq!(seed_14_to_location, 43);
        assert_eq!(seed_55_to_location, 86);
        assert_eq!(seed_13_to_location, 35);
    }
}
