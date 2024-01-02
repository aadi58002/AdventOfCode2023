fn part_2(input: &str) -> usize {}

fn part_1(input: &str) -> usize {
    let mut count = 0;
    for line in input.lines() {
        let (string, nums) = line.split_once(' ').unwrap();
        let spring = string.chars().collect::<Vec<char>>();
        let continuous = nums
            .split(',')
            .map(|val| val.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
    }
    count
}

fn main() {
    let input = include_str!("./part1-input.txt");
    let output = part_1(input);
    dbg!(output);
    // let output = part_2(input);
    // dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let output = part_1(input);
        assert_eq!(output, 10);
    }

    #[test]
    fn part_2_test() {
        //         let input = "#.##..##.
        // ..#.##.#.
        // ##......#
        // ##......#
        // ..#.##.#.
        // ..##..##.
        // #.#.##.#.
        //
        // #...##..#
        // #....#..#
        // ..##..###
        // #####.##.
        // #####.##.
        // ..##..###
        // #....#..#";
        //         let output = part_2(input);
        //         assert_eq!(output, 400);
    }
}
