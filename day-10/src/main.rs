use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn get(&self, matrix: &Vec<Vec<char>>) -> Option<char> {
        if self.x < 0 || self.y < 0 {
            return None;
        }
        Some(*matrix.get(self.y as usize)?.get(self.x as usize)?)
    }

    fn action(&mut self, action: &str) {
        let change = match action {
            "up" => (0, 1),
            "down" => (0, -1),
            "right" => (-1, 0),
            "left" => (1, 0),
            _ => panic!("Invalid action"),
        };
        self.x += change.0;
        self.y += change.1;
    }
}

fn part_2(input: &str) -> usize {
    let motions = HashMap::from([
        (("down", '|'), "down"),
        (("up", '|'), "up"),
        (("right", '-'), "right"),
        (("left", '-'), "left"),
        (("up", 'L'), "left"),
        (("right", 'L'), "down"),
        (("up", 'J'), "right"),
        (("left", 'J'), "down"),
        (("down", '7'), "right"),
        (("left", '7'), "up"),
        (("down", 'F'), "left"),
        (("right", 'F'), "up"),
    ]);
    let mut starting_position: Point = Point { x: 0, y: 0 };
    let mut matrix: Vec<Vec<char>> = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut temp = vec![];
        for (x, ch) in line.chars().enumerate() {
            if ch == 'S' {
                starting_position = Point {
                    x: x as isize,
                    y: y as isize,
                };
            }
            temp.push(ch)
        }
        matrix.push(temp);
    }
    let walkers = [
        (
            Point {
                x: starting_position.x,
                y: starting_position.y - 1,
            },
            "down",
        ),
        (
            Point {
                x: starting_position.x,
                y: starting_position.y + 1,
            },
            "up",
        ),
        (
            Point {
                x: starting_position.x + 1,
                y: starting_position.y,
            },
            "left",
        ),
        (
            Point {
                x: starting_position.x - 1,
                y: starting_position.y,
            },
            "right",
        ),
    ];
    let mut polygon = vec![starting_position];
    'outer: for mut walker in walkers {
        while let Some(val) = walker.0.get(&matrix) {
            if let Some(action) = motions.get(&(walker.1, val)) {
                polygon.push(walker.0);
                walker.0.action(*action);
                walker.1 = action;
                if walker.0 == starting_position {
                    polygon.push(walker.0);
                    break 'outer;
                }
            } else {
                break;
            }
        }
        polygon.clear();
    }

    let mut count = 0;
    todo!("Something to check if a point is inside polygon vec points");
    count
}

fn part_1(input: &str) -> usize {
    let motions = HashMap::from([
        (("down", '|'), "down"),
        (("up", '|'), "up"),
        (("right", '-'), "right"),
        (("left", '-'), "left"),
        (("up", 'L'), "left"),
        (("right", 'L'), "down"),
        (("up", 'J'), "right"),
        (("left", 'J'), "down"),
        (("down", '7'), "right"),
        (("left", '7'), "up"),
        (("down", 'F'), "left"),
        (("right", 'F'), "up"),
    ]);
    let mut starting_position: Point = Point { x: 0, y: 0 };
    let mut matrix: Vec<Vec<char>> = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut temp = vec![];
        for (x, ch) in line.chars().enumerate() {
            if ch == 'S' {
                starting_position = Point {
                    x: x as isize,
                    y: y as isize,
                };
            }
            temp.push(ch)
        }
        matrix.push(temp);
    }
    let walkers = [
        (
            Point {
                x: starting_position.x,
                y: starting_position.y - 1,
            },
            "down",
        ),
        (
            Point {
                x: starting_position.x,
                y: starting_position.y + 1,
            },
            "up",
        ),
        (
            Point {
                x: starting_position.x + 1,
                y: starting_position.y,
            },
            "left",
        ),
        (
            Point {
                x: starting_position.x - 1,
                y: starting_position.y,
            },
            "right",
        ),
    ];
    for mut walker in walkers {
        let mut distance = 1;
        while let Some(val) = walker.0.get(&matrix) {
            if let Some(action) = motions.get(&(walker.1, val)) {
                distance += 1;
                walker.0.action(*action);
                walker.1 = action;
                if walker.0 == starting_position {
                    distance += 1;
                    return distance / 2;
                }
            } else {
                break;
            }
        }
    }
    panic!("Didn't find a answer");
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
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";

        let output = part_1(input);
        let input2 = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let output2 = part_1(input2);
        assert_eq!(output, 4);
        assert_eq!(output2, 8);
    }

    #[test]
    fn part_2_test() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        let output = part_2(input);
        let input2 = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let output2 = part_2(input2);
        let input3 = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let output3 = part_2(input3);
        assert_eq!(output, 4);
        assert_eq!(output2, 8);
        assert_eq!(output3, 10);
    }
}
