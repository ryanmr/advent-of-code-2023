use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

use regex::Regex;

fn main() {
    let input = include_str!("input1.txt").trim();

    let grid = parse_grid(input);

    let answer = find_gear_ratio(&grid);

    println!("answer = {}", answer);
    // 76314915
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
enum CellType {
    Blank,
    Value,
    Symbol,
}

#[allow(dead_code)]
#[derive(Debug, Eq)]
struct Cell {
    id: String,

    val: String,
    row: u32,
    col: u32,

    // dead code
    anchor: bool,

    kind: CellType,
}

impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

fn parse_grid(input: &str) -> HashMap<String, Cell> {
    let symbols = "!@#$%^&*()_+=-/";
    let mut map: HashMap<String, Cell> = HashMap::new();

    let lines: Vec<&str> = input.trim().split("\n").map(|s| s.trim()).collect();
    let re = Regex::new(r"(\d+|\.|[!@#$%^&*()\_+=\-\/])").unwrap();

    for (row_index, line) in lines.into_iter().enumerate() {
        let matches = re.find_iter(line);
        let mut col_index = 0;
        for (_, m) in matches.enumerate() {
            let str = m.as_str();
            let len = m.end() - m.start();

            let kind = if symbols.contains(str) {
                CellType::Symbol
            } else if str == "." {
                CellType::Blank
            } else {
                CellType::Value
            };

            let id = if kind == CellType::Value {
                format!("{},{},{}", row_index, col_index, str)
            } else {
                format!("{},{}", row_index, col_index)
            };

            for n in 0..len {
                let cell = Cell {
                    id: id.to_string(),
                    row: row_index as u32,
                    col: col_index,
                    val: str.to_string(),
                    kind,

                    anchor: if len > 1 && n == 0 { true } else { false },
                };

                map.insert(format!("{row_index},{col_index}"), cell);

                col_index = col_index + 1;
            }
        }
    }

    return map;
}

fn find_adjacent_cells(grid: &HashMap<String, Cell>) -> HashSet<&Cell> {
    let mut set: HashSet<&Cell> = HashSet::new();

    for (_, v) in grid.into_iter() {
        if v.kind == CellType::Value {
            // convert these temporarily to i32 so they can go negative
            // and we can just pretend to not find anything or ignore these entirely
            // the grid.get will return an option and can ignore none
            let row = v.row as i32;
            let col = v.col as i32;

            // 1.  2.  3.
            // 8.  *   4.
            // 7.  6.  5.
            let adjacent_cells = vec![
                // 1
                grid.get(&format!("{},{}", row - 1, col - 1)),
                // 2
                grid.get(&format!("{},{}", row - 1, col)),
                // 3
                grid.get(&format!("{},{}", row - 1, col + 1)),
                // 4
                grid.get(&format!("{},{}", row, col + 1)),
                // 5
                grid.get(&format!("{},{}", row + 1, col + 1)),
                // 6
                grid.get(&format!("{},{}", row + 1, col)),
                // 7
                grid.get(&format!("{},{}", row + 1, col - 1)),
                // 8
                grid.get(&format!("{},{}", row, col - 1)),
            ];

            adjacent_cells.into_iter().for_each(|adjacent| {
                if let Some(cell) = adjacent {
                    if cell.kind == CellType::Symbol {
                        set.insert(&v);
                    }
                }
            });
        }
    }

    return set;
}

fn find_gear_ratio(grid: &HashMap<String, Cell>) -> u32 {
    let mut sum = 0;

    for (_, v) in grid.into_iter() {
        if v.kind == CellType::Symbol && v.val == "*" {
            // convert these temporarily to i32 so they can go negative
            // and we can just pretend to not find anything or ignore these entirely
            // the grid.get will return an option and can ignore none
            let row = v.row as i32;
            let col = v.col as i32;

            // 1.  2.  3.
            // 8.  *   4.
            // 7.  6.  5.
            let adjacent_cells = vec![
                // 1
                grid.get(&format!("{},{}", row - 1, col - 1)),
                // 2
                grid.get(&format!("{},{}", row - 1, col)),
                // 3
                grid.get(&format!("{},{}", row - 1, col + 1)),
                // 4
                grid.get(&format!("{},{}", row, col + 1)),
                // 5
                grid.get(&format!("{},{}", row + 1, col + 1)),
                // 6
                grid.get(&format!("{},{}", row + 1, col)),
                // 7
                grid.get(&format!("{},{}", row + 1, col - 1)),
                // 8
                grid.get(&format!("{},{}", row, col - 1)),
            ];

            let mut set: HashSet<&Cell> = HashSet::new();

            adjacent_cells.into_iter().for_each(|f| {
                if let Some(cell) = f {
                    if cell.kind == CellType::Value {
                        set.insert(cell);
                    }
                }
            });

            let list = set.into_iter().collect::<Vec<&Cell>>();

            if list.len() == 2 {
                let first = list.first().unwrap();
                let last = list.last().unwrap();
                let first_val = &first.val.parse::<u32>().unwrap();
                let last_val = &last.val.parse::<u32>().unwrap();
                let mul = first_val * last_val;
                sum = sum + mul;
            }
        }
    }

    return sum;
}

fn find_signature(cells: Vec<&Cell>) -> u32 {
    let sum = cells
        .into_iter()
        .map(|f| f.val.parse::<u32>().unwrap())
        .sum();
    return sum;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        find_adjacent_cells, find_gear_ratio, find_signature, parse_grid, Cell, CellType::*,
    };

    #[test]
    fn it_parses_basic_case() {
        let input = "
        ..123.456...
        .....*......
        ..234.567..."
            .trim();

        let grid = parse_grid(input);

        let expected: HashMap<String, Cell> = HashMap::from([
            (
                "2,10".to_string(),
                Cell {
                    id: "2,10".to_string(),
                    val: ".".to_string(),
                    row: 2,
                    col: 10,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "0,11".to_string(),
                Cell {
                    id: "0,11".to_string(),
                    val: ".".to_string(),
                    row: 0,
                    col: 11,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "2,0".to_string(),
                Cell {
                    id: "2,0".to_string(),
                    val: ".".to_string(),
                    row: 2,
                    col: 0,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "2,6".to_string(),
                Cell {
                    id: "2,6,567".to_string(),
                    val: "567".to_string(),
                    row: 2,
                    col: 6,
                    anchor: true,
                    kind: Value,
                },
            ),
            (
                "1,2".to_string(),
                Cell {
                    id: "1,2".to_string(),
                    val: ".".to_string(),
                    row: 1,
                    col: 2,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "0,8".to_string(),
                Cell {
                    id: "0,6,456".to_string(),
                    val: "456".to_string(),
                    row: 0,
                    col: 8,
                    anchor: false,
                    kind: Value,
                },
            ),
            (
                "1,7".to_string(),
                Cell {
                    id: "1,7".to_string(),
                    val: ".".to_string(),
                    row: 1,
                    col: 7,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "2,2".to_string(),
                Cell {
                    id: "2,2,234".to_string(),
                    val: "234".to_string(),
                    row: 2,
                    col: 2,
                    anchor: true,
                    kind: Value,
                },
            ),
            (
                "2,1".to_string(),
                Cell {
                    id: "2,1".to_string(),
                    val: ".".to_string(),
                    row: 2,
                    col: 1,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "1,4".to_string(),
                Cell {
                    id: "1,4".to_string(),
                    val: ".".to_string(),
                    row: 1,
                    col: 4,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "1,11".to_string(),
                Cell {
                    id: "1,11".to_string(),
                    val: ".".to_string(),
                    row: 1,
                    col: 11,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "1,6".to_string(),
                Cell {
                    id: "1,6".to_string(),
                    val: ".".to_string(),
                    row: 1,
                    col: 6,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "0,4".to_string(),
                Cell {
                    id: "0,2,123".to_string(),
                    val: "123".to_string(),
                    row: 0,
                    col: 4,
                    anchor: false,
                    kind: Value,
                },
            ),
            (
                "2,3".to_string(),
                Cell {
                    id: "2,2,234".to_string(),
                    val: "234".to_string(),
                    row: 2,
                    col: 3,
                    anchor: false,
                    kind: Value,
                },
            ),
            (
                "2,8".to_string(),
                Cell {
                    id: "2,6,567".to_string(),
                    val: "567".to_string(),
                    row: 2,
                    col: 8,
                    anchor: false,
                    kind: Value,
                },
            ),
            (
                "1,1".to_string(),
                Cell {
                    id: "1,1".to_string(),
                    val: ".".to_string(),
                    row: 1,
                    col: 1,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "2,5".to_string(),
                Cell {
                    id: "2,5".to_string(),
                    val: ".".to_string(),
                    row: 2,
                    col: 5,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "0,3".to_string(),
                Cell {
                    id: "0,2,123".to_string(),
                    val: "123".to_string(),
                    row: 0,
                    col: 3,
                    anchor: false,
                    kind: Value,
                },
            ),
            (
                "1,3".to_string(),
                Cell {
                    id: "1,3".to_string(),
                    val: ".".to_string(),
                    row: 1,
                    col: 3,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "0,1".to_string(),
                Cell {
                    id: "0,1".to_string(),
                    val: ".".to_string(),
                    row: 0,
                    col: 1,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "0,6".to_string(),
                Cell {
                    id: "0,6,456".to_string(),
                    val: "456".to_string(),
                    row: 0,
                    col: 6,
                    anchor: true,
                    kind: Value,
                },
            ),
            (
                "0,0".to_string(),
                Cell {
                    id: "0,0".to_string(),
                    val: ".".to_string(),
                    row: 0,
                    col: 0,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "1,10".to_string(),
                Cell {
                    id: "1,10".to_string(),
                    val: ".".to_string(),
                    row: 1,
                    col: 10,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "2,7".to_string(),
                Cell {
                    id: "2,6,567".to_string(),
                    val: "567".to_string(),
                    row: 2,
                    col: 7,
                    anchor: false,
                    kind: Value,
                },
            ),
            (
                "2,11".to_string(),
                Cell {
                    id: "2,11".to_string(),
                    val: ".".to_string(),
                    row: 2,
                    col: 11,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "1,5".to_string(),
                Cell {
                    id: "1,5".to_string(),
                    val: "*".to_string(),
                    row: 1,
                    col: 5,
                    anchor: false,
                    kind: Symbol,
                },
            ),
            (
                "0,2".to_string(),
                Cell {
                    id: "0,2,123".to_string(),
                    val: "123".to_string(),
                    row: 0,
                    col: 2,
                    anchor: true,
                    kind: Value,
                },
            ),
            (
                "0,10".to_string(),
                Cell {
                    id: "0,10".to_string(),
                    val: ".".to_string(),
                    row: 0,
                    col: 10,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "0,9".to_string(),
                Cell {
                    id: "0,9".to_string(),
                    val: ".".to_string(),
                    row: 0,
                    col: 9,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "1,0".to_string(),
                Cell {
                    id: "1,0".to_string(),
                    val: ".".to_string(),
                    row: 1,
                    col: 0,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "0,5".to_string(),
                Cell {
                    id: "0,5".to_string(),
                    val: ".".to_string(),
                    row: 0,
                    col: 5,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "1,9".to_string(),
                Cell {
                    id: "1,9".to_string(),
                    val: ".".to_string(),
                    row: 1,
                    col: 9,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "0,7".to_string(),
                Cell {
                    id: "0,6,456".to_string(),
                    val: "456".to_string(),
                    row: 0,
                    col: 7,
                    anchor: false,
                    kind: Value,
                },
            ),
            (
                "1,8".to_string(),
                Cell {
                    id: "1,8".to_string(),
                    val: ".".to_string(),
                    row: 1,
                    col: 8,
                    anchor: false,
                    kind: Blank,
                },
            ),
            (
                "2,4".to_string(),
                Cell {
                    id: "2,2,234".to_string(),
                    val: "234".to_string(),
                    row: 2,
                    col: 4,
                    anchor: false,
                    kind: Value,
                },
            ),
            (
                "2,9".to_string(),
                Cell {
                    id: "2,9".to_string(),
                    val: ".".to_string(),
                    row: 2,
                    col: 9,
                    anchor: false,
                    kind: Blank,
                },
            ),
        ]);

        // using these to compare the two sets
        let grid_list = grid.keys().collect::<Vec<&String>>().sort();
        let expected_list = expected.keys().collect::<Vec<&String>>().sort();

        assert_eq!(grid_list, expected_list);
    }

    #[test]
    fn it_find_adjacent_diagonal_cells() {
        let input = "
        ..123.456...
        .....*......
        ..234.567..."
            .trim();

        let grid = parse_grid(input);

        let mut actual = find_adjacent_cells(&grid)
            .into_iter()
            .map(|f| &f.id)
            .collect::<Vec<&String>>();
        actual.sort();

        let mut expected = vec!["0,2,123", "2,2,234", "2,6,567", "0,6,456"];
        expected.sort();

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_find_adjacent_cardinal_cells() {
        let input = "
        ....975.....
        ....1*32....
        .....52....."
            .trim();

        let grid = parse_grid(input);

        let mut actual = find_adjacent_cells(&grid)
            .into_iter()
            .map(|f| &f.id)
            .collect::<Vec<&String>>();
        actual.sort();

        let mut expected = vec!["2,5,52", "1,6,32", "1,4,1", "0,4,975"];
        expected.sort();

        assert_eq!(actual, expected)
    }

    #[test]
    fn it_find_no_adjacent_cells() {
        let input = "
        1..........2
        7....*.....3
        6..........4"
            .trim();

        let grid = parse_grid(input);

        let mut actual = find_adjacent_cells(&grid)
            .into_iter()
            .map(|f| &f.id)
            .collect::<Vec<&String>>();
        actual.sort();

        let mut expected: Vec<&str> = vec![];
        expected.sort();

        assert_eq!(actual, expected)
    }

    #[test]
    fn it_finds_signature() {
        let input = "
        ..123.456...
        .....*......
        ..234.567..."
            .trim();

        let grid = parse_grid(input);

        let adjacent_cells = find_adjacent_cells(&grid)
            .into_iter()
            .collect::<Vec<&Cell>>();

        let actual = find_signature(adjacent_cells);

        let expected = 1380;

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_blends_part1() {
        let input = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."
            .trim();

        let grid = parse_grid(input);

        let adjacent_cells = find_adjacent_cells(&grid)
            .into_iter()
            .collect::<Vec<&Cell>>();

        let actual = find_signature(adjacent_cells);

        let expected = 4361;

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_blends_part2() {
        let input = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."
            .trim();

        let grid = parse_grid(input);

        let actual = find_gear_ratio(&grid);

        let expected = 467835;

        assert_eq!(actual, expected);
    }
}
