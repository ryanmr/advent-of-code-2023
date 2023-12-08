use rayon::current_thread_index;
use rayon::prelude::*;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input1.txt");

    let almanac = parse_almanac(input);

    println!("total seeds in the almanac = {}", almanac.seeds.len());

    let seeds = &almanac.seeds;

    let results = seeds
        .clone()
        .into_par_iter()
        .map(|f| find_trace_seed_to_location(&almanac, f))
        .collect::<Vec<u64>>();

    let min = results.into_iter().min().unwrap();

    println!("answer = {}", min);
    // 6082852
    // disclaimer
    // this was brute forced - made time possible with rayon and space possible by m1 (fails on linux)
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

    // in part2, seeds are ranges, "position offset" pairs
    // we could explode the size of this array but it is amusing

    let seed_line_numbers = seeds_line
        .trim()
        .split(" ")
        .map(|f| f.trim())
        .map(|f| f.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut seeds: Vec<u64> = vec![];
    let mut i = 0;
    loop {
        if i >= seed_line_numbers.len() {
            break;
        }

        let position = seed_line_numbers[i];
        let offset = seed_line_numbers[i + 1];

        println!("seed group = {}", position);

        for n in position..(position + offset) {
            seeds.push(n);
        }

        i = i + 2;
    }

    println!("number of seeds = {}", seeds.len());

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

                // println!("forward line: {}", forward_line);

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
    // println!(
    //     "{}: seed = {}",
    //     rayon::current_thread_index().unwrap(),
    //     seed
    // );

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
            // val should equal or greater than source,
            // val should be less than source + range; but NOT less than
            // off by one haunts us all
            if val >= map.source_range_start && val < map.source_range_start + map.range_length {
                // println!(
                //     "{}: {} >= {} && {} < {}",
                //     ord,
                //     val,
                //     map.source_range_start,
                //     val,
                //     map.source_range_start + map.range_length
                // );
                let offset = val - map.source_range_start;
                let dest = map.destination_range_start + offset;

                // println!("{}: mapped source {} -> destination {}", ord, val, dest);
                val = dest;
                break; // stop trying maps from the same category
                       // if it is already found
            } else {
                // println!("{}: mapped source {} -> destination {}", ord, val, val);
            }
        }
    }

    // println!("final val = {}", val);

    return val;
}

#[cfg(test)]
mod tests {
    use crate::{find_trace_seed_to_location, parse_almanac};
    use rayon::prelude::*;

    #[test]
    fn it_blends_long() {
        let input = "
        seeds: 280775197 7535297 3229061264 27275209 77896732 178275214 2748861189 424413807 3663093536 130341162 613340959 352550713 1532286286 1115055792 1075412586 241030710 3430371306 138606714 412141395 146351614

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

        let seeds = &almanac.seeds;

        assert_eq!(2761436232, seeds.len());
    }

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

        let seeds = &almanac.seeds;

        println!("seeds = {:#?}", seeds);

        let results = seeds
            .clone()
            .into_par_iter()
            .map(|f| find_trace_seed_to_location(&almanac, f))
            .collect::<Vec<u64>>();

        let min = results.into_iter().min().unwrap();

        let expected = 46;

        assert_eq!(min, expected);
    }
}
