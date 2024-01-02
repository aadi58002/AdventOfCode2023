use std::collections::HashMap;

fn part_2(input: &str) -> usize {
    let matrixes = input.split("\n\n");
    let mut values = vec![];
    for (num, matrix) in matrixes.enumerate() {
        values.push(match mirror_cols(matrix) {
            Some(cols) => cols,
            None => mirror_rows(matrix).unwrap() * 100,
        })
    }
    values.iter().sum()
}

fn find_mirror_cols(input: Vec<Vec<char>>) -> Option<usize> {
    let input: Vec<Vec<char>> = input.into_iter().filter(|val| (*val).len() != 0).collect();
    let mut diffs = vec![];
    for row in &input {
        let len = row.len();
        for mirror_loc in 0..(len - 1) {
            let from_start = &row[..=mirror_loc];
            let till_end = &row[(mirror_loc + 1)..];
            // Part 1
            // if from_start
            //     .iter()
            //     .rev()
            //     .zip(till_end.iter())
            //     .all(|(val1, val2)| *val1 == *val2)
            // {
            //     diffs.push(mirror_loc + 1);
            // }
            diffs.push((
                mirror_loc + 1,
                from_start
                    .iter()
                    .rev()
                    .zip(till_end.iter())
                    .filter(|(val1, val2)| **val1 != **val2)
                    .count(),
            ));
        }
    }

    // part 1
    // let freq = diffs.iter().fold(HashMap::new(), |mut map, val| {
    //     map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
    //     map
    // });

    let freq = diffs
        .iter()
        .fold(HashMap::new(), |mut map, (mirror, diffs)| {
            map.entry(mirror).and_modify(|(freq,diff_count): &mut (usize, usize)| {
                *freq += 1;
                *diff_count += *diffs;
            }).or_insert((1,*diffs));
            map
        });

    for (key, (mirror, diff)) in freq {
        //part 1
        // if value == input.len() {
        //part 2
        if mirror == input.len() && diff == 1 {
            return Some(*key);
        }
    }
    None
}

fn mirror_cols(input: &str) -> Option<usize> {
    let mut matrix = vec![];
    for row in input.split('\n') {
        let mut temp = Vec::with_capacity(100);
        for ch in row.chars() {
            temp.push(ch);
        }
        matrix.push(temp);
    }
    find_mirror_cols(matrix)
}

fn mirror_rows(input: &str) -> Option<usize> {
    let rows: Vec<&str> = input.split('\n').collect();
    let mut matrix = vec![vec![]; rows[0].len()];
    for x in 0..rows.len() {
        for (y, ch) in rows[x].chars().enumerate() {
            matrix[y].push(ch);
        }
    }
    find_mirror_cols(matrix)
}

fn part_1(input: &str) -> usize {
    let matrixes = input.split("\n\n");
    let mut values = vec![];
    for matrix in matrixes {
        values.push(match mirror_cols(matrix) {
            Some(cols) => cols,
            None => mirror_rows(matrix).unwrap() * 100,
        })
    }
    values.iter().sum()
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
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let output = part_1(input);
        assert_eq!(output, 405);
    }

    #[test]
    fn part_2_test() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let output = part_2(input);
        assert_eq!(output, 400);
    }
}
