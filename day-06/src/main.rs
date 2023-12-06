fn part_2(input: &str) -> usize {
    let mut lines = input.lines().map(|val| val.replace(" ",""));
    let time = lines.nth(0).unwrap().split(":").skip(1).nth(0).unwrap().parse::<usize>().unwrap();
    let distance = lines.nth(0).unwrap().split(":").skip(1).nth(0).unwrap().parse::<usize>().unwrap();
    
    let mut answer = 0;
    for i in 1..time{
        if i*(time-i) > distance{
            answer += 1;
        }
    }
    answer
}

// I am lazy and don't want to optimize it (optimizations i can think of -> Using a custom binary
// search and search till just time/2 in each case)
fn part_1(input: &str) -> usize {
    let mut lines = input.lines();
    let time = lines.nth(0).unwrap().split_whitespace().skip(1).map(|val| val.parse::<usize>().unwrap());
    let distance = lines.nth(0).unwrap().split_whitespace().skip(1).map(|val| val.parse::<usize>().unwrap());
    
    let mut answer = 1;
    for (time,distance) in time.zip(distance){
        let mut counter = 0;
        for i in 1..time{
            if i*(time-i) > distance{
                counter += 1;
            }
        }
        answer *= counter;
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
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let output = part_1(input);
        assert_eq!(output, 288);
    }

    #[test]
    fn part_2_test() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let output = part_2(input);
        assert_eq!(output,71503);
    }
}
