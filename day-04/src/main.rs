fn part_2(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let length = lines.len();
    let mut copies = vec![1; length];
    for (index, line) in lines.into_iter().enumerate() {
        let mut str_arr = line.split(':').nth(1).unwrap().split('|');
        let arr1: Vec<usize> = str_arr
            .nth(0)
            .unwrap()
            .split_whitespace()
            .map(|val| val.parse::<usize>().unwrap())
            .collect();
        let arr2: Vec<usize> = str_arr
            .nth(0)
            .unwrap()
            .split_whitespace()
            .map(|val| val.parse::<usize>().unwrap())
            .collect();
        let mut matches = 0;
        for val1 in &arr1 {
            for val2 in &arr2 {
                if val1 == val2 {
                    matches += 1;
                }
            }
        }
        for i in 1..=matches {
            let val = i + index;
            if val < length {
                copies[val] += copies[index];
            }
        }
    }
    dbg!(&copies);
    copies.into_iter().sum()
}

fn part_1(input: &str) -> usize {
    let mut sum = 0;
    for line in input.lines() {
        let mut str_arr = line.split(':').nth(1).unwrap().split('|');
        let arr1: Vec<usize> = str_arr
            .nth(0)
            .unwrap()
            .split_whitespace()
            .map(|val| val.parse::<usize>().unwrap())
            .collect();
        let arr2: Vec<usize> = str_arr
            .nth(0)
            .unwrap()
            .split_whitespace()
            .map(|val| val.parse::<usize>().unwrap())
            .collect();
        let mut matches = 0;
        for val1 in &arr1 {
            for val2 in &arr2 {
                if val1 == val2 {
                    matches += 1;
                }
            }
        }
        if matches != 0 {
            sum += 2_i32.pow(matches - 1) as usize;
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
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let output = part_1(input);
        assert_eq!(output, 13);
    }

    #[test]
    fn part_2_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let output = part_2(input);
        assert_eq!(output, 30);
    }
}
