use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MAIN: Regex = Regex::new(r"Game (\d+):(.*)").unwrap();
    static ref VALUE: Regex = Regex::new(r"(\d+) (\w+)").unwrap();
}

#[derive(Debug)]
struct GameResult {
    number: usize,
    red: usize,
    green: usize,
    blue: usize,
}

fn get_number_from_game(value: &str) -> GameResult {
    let games = MAIN.captures(value).unwrap();
    let number = games.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let results = games.get(2).unwrap().as_str();
    let (mut red, mut green, mut blue) = (0, 0, 0);

    for (_, [value, word]) in VALUE.captures_iter(results).map(|c| c.extract()) {
        let value = value.parse::<usize>().unwrap();
        match word {
            "red" => red = red.max(value),
            "green" => green = green.max(value),
            "blue" => blue = blue.max(value),
            _ => panic!("This case should never happen"),
        }
    }

    GameResult {
        number,
        red,
        green,
        blue,
    }
}

fn part_2(input: &str) -> usize {
    let mut sum = 0;
    for input in input.lines() {
        let values = get_number_from_game(input);
        sum += values.red * values.green * values.blue;
    }
    sum
}

fn part_1(input: &str, limit: GameResult) -> usize {
    let mut sum = 0;
    for input in input.lines() {
        let values = get_number_from_game(input);
        dbg!(&values);
        if values.red <= limit.red && values.green <= limit.green && values.blue <= limit.blue {
            sum += values.number;
        }
    }
    sum
}

fn main() {
    // let input = include_str!("./input.txt");
    let input = include_str!("./part1-input.txt");
    // let output = part_1(input,GameResult {
    //     number: 0,
    //     red: 12,
    //     green: 13,
    //     blue: 14,
    // });
    let output = part_2(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                     Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                     Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                     Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                     Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let output = part_1(
            input,
            GameResult {
                number: 0,
                red: 12,
                green: 13,
                blue: 14,
            },
        );
        assert_eq!(output, 8);
    }

    #[test]
    fn part_2_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                     Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                     Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                     Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                     Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let output = part_2(input);
        assert_eq!(output, 2286);
    }
}
