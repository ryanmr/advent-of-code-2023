fn main() {
    let input = include_str!("input1.txt");

    let parsed = parse(input);

    let races = permute_races(parsed.0);
    let winners = find_race_winners(races, parsed.1);

    println!("answer = {}", winners.len());
    // 28228952
}

fn parse(input: &str) -> (u64, u64) {
    let lines = input
        .trim()
        .split("\n")
        .map(|s| s.trim())
        .collect::<Vec<&str>>();

    let time = lines[0]
        .trim()
        .replace("Time:", "")
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    let distance = lines[1]
        .trim()
        .replace("Distance:", "")
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    return (time, distance);
}

fn get_speed(time: u64) -> u64 {
    return time;
}

fn get_distance_covered(speed: u64, time: u64) -> u64 {
    // [speed * time = distance]
    let dist = speed * time;
    return dist;
}

#[derive(Debug, PartialEq, Eq)]
struct Race {
    charge_time: u64,
    distance_covered: u64,
}

fn permute_races(max_time: u64) -> Vec<Race> {
    // generate the values
    // if this was like day 5 it could explode
    // eq?

    let mut races: Vec<Race> = vec![];

    for n in 0..(max_time + 1) {
        let charge_time = n;
        let speed = get_speed(n);

        let time_remaining = max_time - charge_time;
        let distance_covered = get_distance_covered(speed, time_remaining);

        let race = Race {
            charge_time,
            distance_covered,
        };

        races.push(race);
    }

    return races;
}

fn find_race_winners(races: Vec<Race>, threshold: u64) -> Vec<Race> {
    return races
        .into_iter()
        .filter(|race| race.distance_covered > threshold)
        .collect::<Vec<Race>>();
}

#[cfg(test)]
mod tests {
    use crate::{find_race_winners, parse, permute_races, Race};

    #[test]
    fn it_runs_with_7_9() {
        let actual = permute_races(7);

        let expected = vec![
            Race {
                charge_time: 0,
                distance_covered: 0,
            },
            Race {
                charge_time: 1,
                distance_covered: 6,
            },
            Race {
                charge_time: 2,
                distance_covered: 10,
            },
            Race {
                charge_time: 3,
                distance_covered: 12,
            },
            Race {
                charge_time: 4,
                distance_covered: 12,
            },
            Race {
                charge_time: 5,
                distance_covered: 10,
            },
            Race {
                charge_time: 6,
                distance_covered: 6,
            },
            Race {
                charge_time: 7,
                distance_covered: 0,
            },
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_runs_with_7_9_find_winners() {
        let races = permute_races(7);
        let actual = find_race_winners(races, 9);

        println!("actual = {:#?}", actual);

        let expected = vec![
            Race {
                charge_time: 2,
                distance_covered: 10,
            },
            Race {
                charge_time: 3,
                distance_covered: 12,
            },
            Race {
                charge_time: 4,
                distance_covered: 12,
            },
            Race {
                charge_time: 5,
                distance_covered: 10,
            },
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_parses() {
        let input = "
        Time:      7  15   30
        Distance:  9  40  200"
            .trim();

        let actual = parse(input);

        let a = 71530;
        let b = 940200;

        assert_eq!(actual.0, a);
        assert_eq!(actual.1, b);
    }

    #[test]
    fn it_blends() {
        let input = "
        Time:      7  15   30
        Distance:  9  40  200"
            .trim();

        let parsed = parse(input);

        let races = permute_races(parsed.0);
        let winners = find_race_winners(races, parsed.1);

        assert_eq!(winners.len(), 71503);
    }
}
