use std::fs;

struct Coordinates {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

pub fn solution() -> (i32, i32) {
    let input = fs::read_to_string("assets/dive.txt").unwrap();

    (first(&input), second(&input))
}

fn first(input: &str) -> i32 {
    let mut coord = Coordinates {
        depth: 0,
        horizontal: 0,
        aim: 0,
    };

    for line in input.lines() {
        let command: Vec<_> = line.split_whitespace().collect();

        let value: _ = &command[1].parse().unwrap();

        match command[0] {
            "up" => coord.depth -= value,
            "down" => coord.depth += value,
            "forward" => coord.horizontal += value,
            _ => panic!("Received unknown command {line}"),
        }
    }

    coord.depth * coord.horizontal
}

fn second(input: &str) -> i32 {
    let mut coord = Coordinates {
        depth: 0,
        horizontal: 0,
        aim: 0,
    };
    for line in input.lines() {
        let command: Vec<_> = line.split_whitespace().collect();
        let value: _ = &command[1].parse().unwrap();

        match command[0] {
            "up" => coord.aim -= value,
            "down" => coord.aim += value,
            "forward" => {
                coord.horizontal += value;
                coord.depth += coord.aim * value;
            }
            _ => panic!("Received unknown command {line}"),
        }
    }

    coord.depth * coord.horizontal
}
