#[derive(Debug)]
struct MapRanges {
    to: usize,
    from: usize,
    size: usize,
}

impl MapRanges {
    fn get_mapping(self: &Self, from_val: usize) -> Option<usize> {
        let Self { to, from, size } = *self;
        if (from..(from + size)).contains(&from_val) {
            Some(to + from_val - from)
        } else {
            None
        }
    }
}

fn part_2(input: &str) -> usize {
    let mut chunks = input.split("\n\n");
    let seeds: Vec<usize> = chunks
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|val| val.parse::<usize>().unwrap())
        .collect();

    let mut maps_vector = vec![];
    for chunk in chunks {
        let mut maps = vec![];
        for values in chunk.lines().skip(1) {
            let values: Vec<usize> = values
                .split_whitespace()
                .map(|val| val.parse::<usize>().unwrap())
                .collect();
            maps.push(MapRanges {
                to: values[0],
                from: values[1],
                size: values[2],
            });
        }
        maps_vector.push(maps);
    }

    let mut answer = usize::MAX;
    for seeds_range in seeds.chunks(2) {
        for seed in seeds_range[0]..(seeds_range[0]+seeds_range[1]) {
            let mut current = seed;
            for maps in &maps_vector {
                if let Some(val) = maps
                    .iter()
                    .filter_map(|map| map.get_mapping(current))
                    .nth(0)
                {
                    current = val;
                }
            }
            answer = answer.min(current);
        }
    }
    answer
}

fn part_1(input: &str) -> usize {
    let mut chunks = input.split("\n\n");
    let seeds: Vec<usize> = chunks
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|val| val.parse::<usize>().unwrap())
        .collect();

    let mut maps_vector = vec![];
    for chunk in chunks {
        let mut maps = vec![];
        for values in chunk.lines().skip(1) {
            let values: Vec<usize> = values
                .split_whitespace()
                .map(|val| val.parse::<usize>().unwrap())
                .collect();
            maps.push(MapRanges {
                to: values[0],
                from: values[1],
                size: values[2],
            });
        }
        maps_vector.push(maps);
    }

    let mut answer = usize::MAX;
    for seed in seeds {
        let mut current = seed;
        for maps in &maps_vector {
            if let Some(val) = maps
                .iter()
                .filter_map(|map| map.get_mapping(current))
                .nth(0)
            {
                current = val;
            }
        }
        answer = answer.min(current);
    }
    answer
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
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let output = part_1(input);
        assert_eq!(output, 35);
    }

    #[test]
    fn part_2_test() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let output = part_2(input);
        assert_eq!(output, 46);
    }
}
