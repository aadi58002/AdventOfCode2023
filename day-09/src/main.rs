fn part_2(input: &str) -> isize {
    let mut sum = 0;
    for line in input.lines() {
        let mut diff: Vec<Vec<isize>> = vec![];
        diff.push(
            line.split_whitespace()
                .map(|val| {
                    if val.starts_with('-') {
                        val[1..].parse::<isize>().unwrap() * -1
                    } else {
                        val.parse::<isize>().unwrap()
                    }
                })
                .collect(),
        );
        while diff.last().unwrap().iter().any(|val| *val != 0) {
            diff.push(
                diff.last()
                    .unwrap()
                    .windows(2)
                    .map(|val| val[1] - val[0])
                    .collect(),
            );
        }
        let mut sign = 1;
        sum += diff.iter().skip(1).fold(diff[0][0], |acc, val| {
            sign *= -1;
            acc + sign * val.first().unwrap_or(&0)
        });
    }
    sum
}

fn part_1(input: &str) -> isize {
    let mut sum = 0;
    for line in input.lines() {
        let mut diff: Vec<Vec<isize>> = vec![];
        diff.push(
            line.split_whitespace()
                .map(|val| {
                    if val.starts_with('-') {
                        val[1..].parse::<isize>().unwrap() * -1
                    } else {
                        val.parse::<isize>().unwrap()
                    }
                })
                .collect(),
        );
        while diff.last().unwrap().iter().any(|val| *val != 0) {
            diff.push(
                diff.last()
                    .unwrap()
                    .windows(2)
                    .map(|val| val[1] - val[0])
                    .collect(),
            );
        }
        sum += diff
            .iter()
            .fold(0, |acc, val| acc + val.last().unwrap_or(&0));
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
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let output = part_1(input);
        assert_eq!(output, 114);
    }

    #[test]
    fn part_2_test() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let output = part_2(input);
        assert_eq!(output, 2);
    }
}
