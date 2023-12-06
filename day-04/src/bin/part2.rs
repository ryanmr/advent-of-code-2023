use log::{debug, info};
use std::{
    cmp,
    collections::{BTreeMap, HashSet},
};

fn main() {
    let input = include_str!("input1.txt").trim();

    let original_cards = get_cards(input);

    let map = find_derived_cards(original_cards);

    let sum = map.values().sum::<u32>();

    println!("answer map = {:#?}", map);
    println!("answer sum = {}", sum);
    // 11787590
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Card {
    id: String,
    winning: Vec<u32>,
    drawn: Vec<u32>,
}

fn parse_line(input: &str) -> Card {
    let part = input.split(":").map(|f| f.trim()).collect::<Vec<&str>>();
    let card_part = part.first().unwrap();
    let card_id = card_part.replace("Card", "").trim().to_string();

    let numbers_part = part.last().unwrap();
    let number_parts = numbers_part
        .split("|")
        .map(|f| f.trim())
        .collect::<Vec<&str>>();

    let winning_part = number_parts.first().unwrap();
    let drawn_part = number_parts.last().unwrap();

    let winning_numbers = winning_part
        .split_whitespace()
        .map(|f| f.trim())
        .map(|f| f.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let drawn_numbers = drawn_part
        .split_whitespace()
        .map(|f| f.trim())
        .map(|f| f.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    return Card {
        id: card_id,
        winning: winning_numbers,
        drawn: drawn_numbers,
    };
}

fn get_cards(input: &str) -> Vec<Card> {
    let lines = input.split("\n").map(|f| f.trim()).collect::<Vec<&str>>();

    let cards: Vec<Card> = lines.into_iter().map(|f| parse_line(f)).collect();

    return cards;
}

fn get_score_from_card(card: Card) -> u32 {
    // intersection
    let winning_set: HashSet<u32> = card.winning.into_iter().collect();
    let drawn_set: HashSet<u32> = card.drawn.into_iter().collect();

    let intersection = drawn_set.intersection(&winning_set);

    let intersection_list = intersection.into_iter().collect::<Vec<&u32>>();
    let intersection_len = intersection_list.len();

    let score = if intersection_len == 0 {
        0
    } else {
        2_i32.pow(cmp::max(intersection_len as i32 - 1_i32, 0_i32) as u32)
    };

    return score as u32;
}

fn find_match_count(card: &Card) -> u32 {
    let winning_set: HashSet<u32> = card.winning.clone().into_iter().collect();
    let drawn_set: HashSet<u32> = card.drawn.clone().into_iter().collect();

    let intersection = drawn_set.intersection(&winning_set);

    let intersection_list = intersection.into_iter().collect::<Vec<&u32>>();
    let intersection_len = intersection_list.len();

    return intersection_len as u32;
}

fn find_signature(scores: Vec<u32>) -> u32 {
    return scores.into_iter().sum();
}

fn find_derived_cards(cards: Vec<Card>) -> BTreeMap<u32, u32> {
    // BTreeMap is an ordered Map
    // https://doc.rust-lang.org/stable/std/collections/struct.BTreeMap.html
    // this really only for debugging purposes
    let mut map: BTreeMap<u32, u32> = BTreeMap::new();

    // prime the cards initially
    for card in cards.iter() {
        let id_as_number = card.id.clone().parse::<u32>().unwrap();
        map.insert(id_as_number, 1);
    }

    // each iteration should do things to the sums of cards
    for card in cards.into_iter() {
        let id_as_number = card.id.clone().parse::<u32>().unwrap();

        let match_count = find_match_count(&card);

        if match_count == 0 {
            continue;
        }

        let copies_of_current_card = map.get(&id_as_number).unwrap().clone();

        debug!("card {} has {} matches", id_as_number, match_count);

        debug!(
            "card {} has {} copies",
            id_as_number, copies_of_current_card
        );

        for _ in 0..copies_of_current_card {
            let earned_range = (id_as_number + 1)..(id_as_number + match_count + 1);
            for earned_card in earned_range {
                let current_value = map.get(&earned_card).unwrap();
                let next_value = current_value + 1;
                debug!(
                    "earning copy of card {}; {} → {}",
                    earned_card, current_value, next_value
                );
                map.insert(earned_card, next_value);
            }
        }
    }

    return map;
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use log::debug;

    use crate::{
        find_derived_cards, find_signature, get_cards, get_score_from_card, parse_line, Card,
    };

    #[test]
    fn it_parse_line() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".trim();
        let expected = Card {
            id: "1".to_string(),
            winning: vec![41, 48, 83, 86, 17],
            drawn: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        let actual = parse_line(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_get_score_from_card1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".trim();

        let card = parse_line(input);
        let actual = get_score_from_card(card);

        let expected = 8;

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_get_score_from_card2() {
        let input = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".trim();

        let card = parse_line(input);
        let actual = get_score_from_card(card);

        let expected = 2;

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_get_cards() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"
            .trim();
        let expected = vec![
            Card {
                id: "1".to_string(),
                winning: vec![41, 48, 83, 86, 17],
                drawn: vec![83, 86, 6, 31, 17, 9, 48, 53],
            },
            Card {
                id: "2".to_string(),
                winning: vec![13, 32, 20, 16, 61],
                drawn: vec![61, 30, 68, 82, 17, 32, 24, 19],
            },
        ];
        let actual = get_cards(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_blends_part1() {
        let input = "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .trim();

        let scores = get_cards(input)
            .into_iter()
            .map(|card| get_score_from_card(card))
            .collect::<Vec<u32>>();

        let actual = find_signature(scores);
        let expected = 13;

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_blends_part2() {
        let input = "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .trim();

        let original_cards = get_cards(input);

        let map = find_derived_cards(original_cards);

        let sum = map.values().sum::<u32>();

        let expected_sum = 30;

        // expected
        let expected_map = BTreeMap::from([(1, 1), (2, 2), (3, 4), (4, 8), (5, 14), (6, 1)]);

        debug!("actual map = {:#?}", map);
        debug!("actual sum = {}", sum);

        debug!("expected map = {:#?}", expected_map);
        debug!("expected sum = {}", expected_sum);

        assert_eq!(sum, expected_sum);
        assert_eq!(map, expected_map);
    }
}
