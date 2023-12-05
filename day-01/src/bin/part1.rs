fn main() {
    // https://adventofcode.com/2023/day/1/input
    let input1 = include_str!("./input1.txt");
    let answer = part1(input1);
    println!("answer = {}", answer);
    // 55172
}

fn part1(input: &str) -> u32 {
    let lines = input.split("\n").map(str::trim);
    let mut total = 0;
    for line in lines.into_iter() {
        let sum = process_line(line);
        total = total + sum;
    }
    println!("total = {}", total);
    return total;
}

fn process_line(line: &str) -> u32 {
    let first = get_digit(line, Dir::First);
    let last = get_digit(line, Dir::Last);
    let combine = first.to_string() + last.to_string().as_str();
    let number = combine.parse::<u32>().unwrap();
    return number;
}

enum Dir {
    First,
    Last,
}

fn get_digit(input: &str, dir: Dir) -> u32 {
    println!("{}", input);
    
    for index in 0..input.len() {
        println!("{}", index);
        let mut chars = input.chars();
        let c = match dir {
            Dir::First => chars.nth(index).unwrap(),
            Dir::Last => chars.nth_back(index).unwrap()
        };

        let n = c.to_digit(10);
        if let Some(value) = n {
            return value
        }
    }

    return 0
}

#[cfg(test)]
mod tests {
    use crate::{get_digit, process_line};

    #[test]
    fn it_gets_first_digit_1() {
        let input = "1abc2";
        let expect = 1;
        let actual = get_digit(input, crate::Dir::First);
        assert_eq!(actual, expect);
    }
    
    #[test]
    fn it_gets_first_digit_7() {
        let input = "treb7uchet";
        let expect = 7;
        let actual = get_digit(input, crate::Dir::First);
        assert_eq!(actual, expect);
    }
    
    #[test]
    fn it_gets_first_digit_1_again() {
        let input = "a1b2c3d4e5f";
        let expect = 1;
        let actual = get_digit(input, crate::Dir::First);
        assert_eq!(actual, expect);
    }

    #[test]
    fn it_gets_last_digit_5_again() {
        let input = "a1b2c3d4e5f";
        let expect = 5;
        let actual = get_digit(input, crate::Dir::Last);
        assert_eq!(actual, expect);
    }

    fn it_gets_last_digit_2() {
        let input = "1abc2";
        let expect = 2;
        let actual = get_digit(input, crate::Dir::Last);
        assert_eq!(actual, expect);
    }
    
    #[test]
    fn it_gets_last_digit_7() {
        let input = "treb7uchet";
        let expect = 7;
        let actual = get_digit(input, crate::Dir::Last);
        assert_eq!(actual, expect);
    }

    #[test]
    fn it_adds_to_12() {
        let input = "1abc2";
        let expect = 12;
        let actual = process_line(input);
        assert_eq!(actual, expect);
    }

    #[test]
    fn it_adds_to_77() {
        let input = "treb7uchet";
        let expect = 77;
        let actual = process_line(input);
        assert_eq!(actual, expect);
    }

    

    #[test]
    fn it_blends() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        let expect = 142;
        let actual = crate::part1(input);
        assert_eq!(actual, expect);
    }
}
