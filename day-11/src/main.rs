fn part_2(input: &str) -> usize {
    let mut galaxy_points = vec![];
    let mut y_count = 0;
    let mut x_count = 0;
    for (y,line) in input.lines().enumerate(){
        y_count = y;
        for (x,ch) in line.chars().enumerate(){
            x_count =x;
            if ch == '#'{
                galaxy_points.push((x,y));
            }
        }
    }
    let mut rows = vec![1000000;y_count+1];
    let mut columns = vec![1000000;x_count+1];
    for (x,y) in &galaxy_points{
        rows[*y] = 1;
        columns[*x] = 1;
    }
    let mut distance = vec![];
    for (i,g1) in galaxy_points.iter().enumerate(){
        for g2 in galaxy_points[i+1..].iter(){
            let (x_min,x_max,y_min,y_max) = (g1.0.min(g2.0),g1.0.max(g2.0),g1.1.min(g2.1),g1.1.max(g2.1));
            distance.push((x_min..x_max).fold(0,|acc,val| acc + columns[val]) + (y_min..y_max).fold(0,|acc,val| acc + rows[val]));
        }

    }
    distance.iter().sum()
}

fn part_1(input: &str) -> usize{
    let mut galaxy_points = vec![];
    let mut y_count = 0;
    let mut x_count = 0;
    for (y,line) in input.lines().enumerate(){
        y_count = y;
        for (x,ch) in line.chars().enumerate(){
            x_count =x;
            if ch == '#'{
                galaxy_points.push((x,y));
            }
        }
    }
    let mut rows = vec![2;y_count+1];
    let mut columns = vec![2;x_count+1];
    for (x,y) in &galaxy_points{
        rows[*y] = 1;
        columns[*x] = 1;
    }
    let mut distance = vec![];
    for (i,g1) in galaxy_points.iter().enumerate(){
        for g2 in galaxy_points[i+1..].iter(){
            let (x_min,x_max,y_min,y_max) = (g1.0.min(g2.0),g1.0.max(g2.0),g1.1.min(g2.1),g1.1.max(g2.1));
            distance.push((x_min..x_max).fold(0,|acc,val| acc + columns[val]) + (y_min..y_max).fold(0,|acc,val| acc + rows[val]));
        }

    }
    distance.iter().sum()
}

fn main() {
    let input = include_str!("./part1-input.txt");
    let output = part_1(input);
    dbg!(output);
    let output = part_2(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
let output = part_1(input);
        assert_eq!(output, 374);
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
