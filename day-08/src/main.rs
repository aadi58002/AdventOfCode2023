use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref GET_PATHS: Regex = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
}

fn part_2(input: &str) -> usize {
    let mut lines = input.lines();
    let instruction = lines.next().unwrap().chars().cycle();

    let mut roads: HashMap<&str, (&str, &str)> = HashMap::new();
    // remove the empty line
    for line in lines.skip(1) {
        let capture = GET_PATHS.captures(line).unwrap();
        roads.insert(
            capture.get(1).unwrap().as_str(),
            (
                capture.get(2).unwrap().as_str(),
                capture.get(3).unwrap().as_str(),
            ),
        );
    }
    roads
        .keys()
        .filter(|val| (**val).ends_with('A'))
        .map(|val| {
            let mut current = *val;
            let mut steps = 0;
            for ins in instruction.clone() {
                if !current.ends_with('Z') {
                    if ins == 'L' {
                        current = roads[&current].0;
                    } else {
                        current = roads[&current].1;
                    }
                } else {
                    break;
                }
                steps += 1;
            }
            steps
        })
        .fold(1, |acc, val| num_integer::lcm(acc, val))
}

fn part_1(input: &str) -> usize {
    let mut lines = input.lines();
    let instruction = lines.next().unwrap().chars().cycle();

    let mut roads: HashMap<&str, (&str, &str)> = HashMap::new();
    // remove the empty line
    for line in lines.skip(1) {
        let capture = GET_PATHS.captures(line).unwrap();
        roads.insert(
            capture.get(1).unwrap().as_str(),
            (
                capture.get(2).unwrap().as_str(),
                capture.get(3).unwrap().as_str(),
            ),
        );
    }

    let mut steps = 0;
    let mut current = "AAA";
    for ins in instruction {
        if current != "ZZZ" {
            let value = roads[current];
            if ins == 'L' {
                current = value.0;
            } else {
                current = value.1;
            }
            steps += 1;
        } else {
            break;
        }
    }
    steps
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
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let input2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let output = part_1(input);
        let output2 = part_1(input2);
        assert_eq!(output, 2);
        assert_eq!(output2, 6);
    }

    #[test]
    fn part_2_test() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let output = part_2(input);
        assert_eq!(output, 6);
    }
}
