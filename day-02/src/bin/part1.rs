use std::collections::HashSet;

fn main() {
    // https://adventofcode.com/2023/day/2
    println!("Hello, world!");

    let input1 = include_str!("input1.txt").trim();
    // only 12 red cubes, 13 green cubes, and 14 blue cubes

    let games: Vec<Game> = input1
        .split("\n")
        .map(|s| s.trim())
        .map(|s| parse_game(s))
        .collect();

    let analysis = analyze_games(games, 12, 13, 14);
    // 2632
}

#[derive(Debug)]
struct GameAnalysis {
    possible_ids: Vec<u32>,
    impossible_ids: Vec<u32>,
    possible_signature: u32,
}

fn analyze_games(
    games: Vec<Game>,
    red_threshold: u32,
    green_threshold: u32,
    blue_threshold: u32,
) -> GameAnalysis {
    // let mut possible_ids: Vec<u32> = vec![];
    // let mut impossible_ids: Vec<u32> = vec![];
    let mut possible_ids_set: HashSet<u32> = HashSet::new();
    let mut impossible_ids_set: HashSet<u32> = HashSet::new();

    for game in games.into_iter() {
        println!("analyze game id = {}", game.id);

        // if any set does not meet the criteria, the game was not possible
        let mut is_game_possible = true;
        for set in game.sets.into_iter() {
            let cond = set.blue <= blue_threshold
                && set.red <= red_threshold
                && set.green <= green_threshold;
            if !cond {
                is_game_possible = false;
            }
        }

        if is_game_possible {
            possible_ids_set.insert(game.id);
        } else {
            impossible_ids_set.insert(game.id);
        }
    }

    let possible_ids: Vec<u32> = possible_ids_set.into_iter().collect();
    let impossible_ids: Vec<u32> = impossible_ids_set.into_iter().collect();

    let possible_signature: u32 = possible_ids.iter().sum();

    let analysis = GameAnalysis {
        possible_ids: possible_ids,
        impossible_ids: impossible_ids,
        possible_signature,
    };

    println!("possible game ids = {:#?}", analysis.possible_ids);
    println!("impossible game ids = {:#?}", analysis.impossible_ids);
    println!("possible signature = {:#?}", analysis.possible_signature);

    return analysis;
}

#[derive(Debug, PartialEq)]
struct CubeSet {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

fn parse_game(input: &str) -> Game {
    let colon_parts: Vec<&str> = input.split(":").collect();
    // left contains the game id
    let left = colon_parts.first().unwrap().trim();

    // right contains the sets
    let right = colon_parts.last().unwrap().trim();

    println!("left = {:#?}", left);
    println!("right = {:#?}", right);

    let game_id_str = left.to_lowercase().replace("game ", "");
    let game_id = game_id_str.parse::<u32>().unwrap();

    println!("game id = {:#?}", game_id);

    let sets: Vec<&str> = right.split(";").map(|s| s.trim()).collect();

    let game_sets: Vec<CubeSet> = sets.into_iter().map(|d| parse_set(d)).collect();

    return Game {
        id: game_id,
        sets: game_sets,
    };
}

fn parse_set(input: &str) -> CubeSet {
    let parts: Vec<&str> = input.split(",").map(|s| s.trim()).collect();
    // per part use a conditional to figure out which one it is?
    let mut set = CubeSet {
        red: 0,
        blue: 0,
        green: 0,
    };
    for part in parts.into_iter() {
        if part.contains("green") {
            let g = part.replace("green", "").trim().parse::<u32>().unwrap();
            set.green = g;
        } else if part.contains("blue") {
            let b = part.replace("blue", "").trim().parse::<u32>().unwrap();
            set.blue = b;
        } else if part.contains("red") {
            let r = part.replace("red", "").trim().parse::<u32>().unwrap();
            set.red = r;
        } else {
            dbg!("an unknown cube color was found: {}", part);
        }
    }

    return set;
}

#[cfg(test)]
mod tests {
    use crate::{analyze_games, parse_game, parse_set, CubeSet, Game};

    #[test]
    fn it_parses_game1() {
        let input = "Game 1: 19 blue, 12 red; 19 blue, 2 green, 1 red; 13 red, 11 blue";
        let expect = Game {
            id: 1,
            sets: vec![
                CubeSet {
                    red: 12,
                    blue: 19,
                    green: 0,
                },
                CubeSet {
                    red: 1,
                    blue: 19,
                    green: 2,
                },
                CubeSet {
                    red: 13,
                    blue: 11,
                    green: 0,
                },
            ],
        };
        let actual = parse_game(input);

        assert_eq!(actual.id, expect.id);

        assert_eq!(actual.sets, expect.sets);
    }

    #[test]
    fn it_parses_game3() {
        let input = "Game 3: 3 blue, 2 red, 6 green; 4 blue, 6 green, 1 red; 11 green, 12 blue; 2 red, 6 green, 4 blue; 4 green";
        let expect = Game {
            id: 3,
            sets: vec![
                CubeSet {
                    red: 2,
                    blue: 3,
                    green: 6,
                },
                CubeSet {
                    red: 1,
                    blue: 4,
                    green: 6,
                },
                CubeSet {
                    red: 0,
                    blue: 12,
                    green: 11,
                },
                CubeSet {
                    red: 2,
                    blue: 4,
                    green: 6,
                },
                CubeSet {
                    red: 0,
                    blue: 0,
                    green: 4,
                },
            ],
        };
        let actual = parse_game(input);

        assert_eq!(actual.id, expect.id);

        assert_eq!(actual.sets, expect.sets);
    }

    #[test]
    fn it_parses_set() {
        let input = "19 blue, 12 red, 1 green";
        let expect = CubeSet {
            red: 12,
            blue: 19,
            green: 1,
        };
        let actual = parse_set(input);
        assert_eq!(actual, expect);
    }

    #[test]
    fn it_blends() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .trim();

        let games: Vec<Game> = input
            .split("\n")
            .map(|s| s.trim())
            .map(|s| parse_game(s))
            .collect();

        let analysis = analyze_games(games, 12, 13, 14);

        let expect = 8;

        assert_eq!(analysis.possible_signature, expect);
    }
}
