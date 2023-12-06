use lazy_static::lazy_static;
use itertools::Itertools;
use regex::Regex;

lazy_static! {
    static ref NUMBER: Regex = Regex::new(r"\d+").unwrap();
}

fn get_number_values(input: &Vec<&str>, j: usize, i: usize) -> (usize, usize) {
    let numbers = NUMBER.find_iter(input[j]);
    for values in numbers {
        if (values.start()..values.end()).contains(&i) {
            return (values.start(), values.as_str().parse::<usize>().unwrap());
        }
    }
    panic!("This should not happen")
}

fn part_2(input: &str) -> usize {
    let input: Vec<&str> = input.lines().map(|val| val.trim()).collect();
    let mut sum = 0;
    for j in 0..input.len() {
        for i in 0..input[j].len() {
            if input[j].chars().nth(i).unwrap() == '*' {
                let (min_i, max_i, min_j, max_j) = (
                    i.saturating_sub(1),
                    (i + 1) % input[j].len(),
                    j.saturating_sub(1),
                    (j + 1) % input.len(),
                );
                let points: Vec<(usize, usize)> = [
                    (min_i, min_j),
                    (i, min_j),
                    (max_i, min_j),
                    (min_i, max_j),
                    (i, max_j),
                    (max_i, max_j),
                    (min_i, j),
                    (max_i, j),
                ]
                .into_iter()
                .filter(|(i, j)| match input[*j].chars().nth(*i) {
                    Some(val) => ('0'..='9').contains(&val),
                    None => false,
                })
                .map(|(i, j)| get_number_values(&input, j, i))
                .unique()
                .collect();
                if points.len() == 2{
                    sum += points[0].1*points[1].1;
                }
            }
        }
    }
    sum
}

fn check_around(input: &Vec<&str>, j: usize, i: usize) -> bool {
    let min_i = i.saturating_sub(1);
    let max_i = (i + 1) % input[j].len();
    let min_j = j.saturating_sub(1);
    let max_j = (j + 1) % input.len();
    [
        (min_i, min_j),
        (i, min_j),
        (max_i, min_j),
        (min_i, max_j),
        (i, max_j),
        (max_i, max_j),
        (min_i, j),
        (max_i, j),
    ]
    .into_iter()
    .any(|(i, j)| match input[j].chars().nth(i) {
        Some(val) => val != '.' && !('0'..='9').contains(&val),
        None => false,
    })
}

fn part_1(input: &str) -> usize {
    let input: Vec<&str> = input.lines().map(|val| val.trim()).collect();
    let mut sum = 0;
    for (index, line) in input.iter().enumerate() {
        for number in NUMBER.find_iter(&line) {
            if (number.start()..number.end())
                .into_iter()
                .any(|i| check_around(&input, index, i))
            {
                sum += number.as_str().parse::<usize>().unwrap();
            }
        }
    }
    sum
}

fn main() {
    let input = include_str!("./part1-input.txt");
    // let output = part_1(input);
    let output = part_2(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "467..114..
                     ...*......
                     ..35..633.
                     ......#...
                     617*......
                     .....+.58.
                     ..592.....
                     ......755.
                     ...$.*....
                     .664.598..";
        let output = part_1(input);
        assert_eq!(output, 4361);
    }

    #[test]
    fn part_2_test() {
        let input = "467..114..
                     ...*......
                     ..35..633.
                     ......#...
                     617*......
                     .....+.58.
                     ..592.....
                     ......755.
                     ...$.*....
                     .664.598..";
        let output = part_2(input);
        assert_eq!(output, 467835);
    }
}
