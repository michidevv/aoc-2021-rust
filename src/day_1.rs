use std::fs;

pub fn solution() -> (u16, u16) {
    let input = fs::read_to_string("assets/sonarSweep.txt").unwrap();

    (first(&input), second(&input))
}

fn first(input: &str) -> u16 {
    let mut result: u16 = 0;
    let mut prev: u16 = 0;
    for line in input.lines() {
        let current = line.parse::<u16>().unwrap();
        if prev < current && prev > 0 {
            result += 1;
        }

        prev = current;
    }

    result
}

fn second(input: &str) -> u16 {
    let mut result: u16 = 0;

    let mut sum: u16 = 0;
    let lines: Vec<_> = input.lines().collect();
    for (idx, line) in lines.iter().enumerate() {
        let prev = sum;
        sum += line.parse::<u16>().unwrap();
        if idx < 3 {
            continue;
        }

        sum -= lines[idx - 3].parse::<u16>().unwrap();

        if prev < sum {
            result += 1;
        }
    }

    result
}
