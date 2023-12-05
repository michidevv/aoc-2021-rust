mod day_1;
mod day_2;
mod day_3;

fn main() {
    let day_1_result = day_1::solution();
    println!("Result day 1 part 1: {day_1_result:?}");

    let day_2_result = day_2::solution();
    println!("Result day 2: {day_2_result:?}");

    let day_3_result = day_3::solution();
    println!("Result day 3: {day_3_result:?}");
}
