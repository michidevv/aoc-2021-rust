use std::fs;

pub fn solution() -> (i32, i32) {
    let input = fs::read_to_string("./assets/binaryDiagnostic.txt").unwrap();

    (first(&input), second(&input))
}

fn first(input: &str) -> i32 {
    // let mut ones = HashMap::new();
    let mut counter = [0; 12];
    let lines: Vec<_> = input.lines().collect();
    let min_column_len = lines.len() / 2;

    for line in lines {
        for (col, char) in line.chars().enumerate() {
            if char == '1' {
                // ones.entry(col.to_string())
                //     .and_modify(|v| *v += 1)
                //     .or_insert(1);

                counter[col] += 1;
            }
        }
    }
    let bin_str = counter.map(|v| if v > min_column_len { "1" } else { "0" });
    let not_bin_str = bin_str.map(|v| if v == "1" { "0" } else { "1" });

    let x = i32::from_str_radix(&bin_str.join(""), 2).unwrap();
    let y = i32::from_str_radix(&not_bin_str.join(""), 2).unwrap();

    x * y
}

enum RatingType {
    Oxygen,
    CO2,
}

fn second(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().collect();
    let lines_slice = lines.as_slice();
    let oxygen = calc_rating(lines_slice, RatingType::Oxygen);
    let co2 = calc_rating(lines_slice, RatingType::CO2);

    oxygen * co2
}

fn calc_rating(lines: &[&str], rating_type: RatingType) -> i32 {
    let mut ones: Vec<&str> = vec![];
    let mut zeros: Vec<&str> = vec![];
    let mut iter: Vec<&str> = Vec::from(lines);

    let range = 0..iter.first().unwrap().len();

    for idx in range {
        if iter.len() == 1 {
            return i32::from_str_radix(iter.first().unwrap(), 2).unwrap();
        }

        for line in iter {
            if line.chars().nth(idx).unwrap() == '1' {
                ones.push(line);
            } else {
                zeros.push(line);
            }
        }

        let predicate = match rating_type {
            RatingType::Oxygen => ones.len() >= zeros.len(),
            RatingType::CO2 => ones.len() < zeros.len(),
        };

        iter = if predicate {
            Vec::from(ones.as_slice())
        } else {
            Vec::from(zeros.as_slice())
        };

        ones.clear();
        zeros.clear();
    }

    panic!("Not a valid rating details")
}
