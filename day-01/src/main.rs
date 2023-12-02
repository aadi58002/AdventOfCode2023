use std::collections::HashMap;
use lazy_static::lazy_static;
lazy_static! {
    static ref MAPPINGS: HashMap<&'static str, usize> = HashMap::from([
                                                         ("zero", 0),
                                                         ("one", 1),
                                                         ("two", 2),
                                                         ("three", 3),
                                                         ("four", 4),
                                                         ("five", 5),
                                                         ("six", 6),
                                                         ("seven", 7),
                                                         ("eight", 8),
                                                         ("nine", 9),

                                                         ("0", 0),
                                                         ("1", 1),
                                                         ("2", 2),
                                                         ("3", 3),
                                                         ("4", 4),
                                                         ("5", 5),
                                                         ("6", 6),
                                                         ("7", 7),
                                                         ("8", 8),
                                                         ("9", 9),
                                                         ]);
}

fn part_2(input: &str) -> usize {
    let first = regex::Regex::new(r".*?(\d|zero|one|two|three|four|five|six|seven|eight|nine).*").unwrap();
    let last = regex::Regex::new(r".*(\d|zero|one|two|three|four|five|six|seven|eight|nine).*?$").unwrap();
    let mut sum = 0;
    for line in input.lines(){
        let val1 = first.captures(line).unwrap().get(1).unwrap().as_str();
        let val2 = last.captures(line).unwrap().get(1).unwrap().as_str();
        sum += MAPPINGS[val1] * 10 + MAPPINGS[val2]; 
    }
    sum
}

fn part_1(input: &str) -> usize {
    let first = regex::Regex::new(r".*?(\d).*").unwrap();
    let last = regex::Regex::new(r".*(\d).*?$").unwrap();
    let mut sum = 0;
    for line in input.lines(){
        sum += first.captures(line).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap() * 10 + last.captures(line).unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap(); 
    }
    sum
}

fn main() {
    // let input = include_str!("./input.txt");
    let input = include_str!("./part1-input.txt");
    // let input = &input_post_processing(input.to_string());
    let output = part_2(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "1abc2
                    pqr3stu8vwx
                    a1b2c3d4e5f
                    treb7uchet";
        let output = part_1(input);
        assert_eq!(output, 142);
    }

    #[test]
    fn part_2_test() {
        let input = "two1nine
                    eightwothree
                    abcone2threexyz
                    xtwone3four
                    4nineeightseven2
                    zoneight234
                    7pqrstsixteen";
        let output = part_2(input);
        assert_eq!(output,281);
    }
}
