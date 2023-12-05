fn main() {
    // https://adventofcode.com/2023/day/1/input
    // same for part2
    let input1 = include_str!("./input1.txt");
    let answer = part2(input1);
    println!("answer = {}", answer);
    // 54925
}

fn part2(input: &str) -> u32 {
    let lines = input.split("\n").map(str::trim);
    let mut total = 0;
    for line in lines.into_iter() {
        let sum = process_line(line);
        total = total + sum;
    }
    println!("total = {}", total);
    return total;
}

fn combine(first: u32, last: u32) -> u32 {
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

fn process_line(input: &str) -> u32 {
    // handle the word to string conversion
    let i = input
        .replace("one", "o[1]e")
        .replace("two", "t[2]o")
        .replace("three", "t[3]e")
        .replace("four", "f[4]r")
        .replace("five", "f[5]e")
        .replace("six", "s[6]x")
        .replace("seven", "s[7]n")
        .replace("eight", "e[8]t")
        .replace("nine", "n[9]e");

    let first = get_digit(&i, Dir::First);
    let last = get_digit(&i, Dir::Last);
    let combine = combine(first, last);

    return combine;
}

#[cfg(test)]
mod tests {
    use crate::process_line;



    #[test]
    fn it_gets_digits_29() {
        let input = "two12nine";
        let expect = 29;
        let actual = process_line(input);
        assert_eq!(actual, expect);
    }

    #[test]
    fn it_gets_digits_42() {
        let input = "4nineeightseven2";
        let expect = 42;
        let actual = process_line(input);
        assert_eq!(actual, expect);
    }

    #[test]
    fn it_gets_digits_76() {
        let input = "7pqrstsixteen";
        let expect = 76;
        let actual = process_line(input);
        assert_eq!(actual, expect);
    }

    #[test]
    fn it_gets_digits_0_no_numbers() {
        let input = "abced";
        let expect = 0;
        let actual = process_line(input);
        assert_eq!(actual, expect);
    }

    #[test]
    fn it_gets_digits_18_combo() {
        let input = "oneight";
        let expect = 18;
        let actual = process_line(input);
        assert_eq!(actual, expect);
    }

    #[test]
    fn it_blends() {
        let input2 = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        let expect = 281;
        let actual = crate::part2(input2);
        assert_eq!(actual, expect);
    }
}
