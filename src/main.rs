// mod day1;
// mod day2;
mod day3;
// mod day4;
// mod day5;


pub fn read_file(filepath: &str) -> String {
    let contents = std::fs::read_to_string(filepath);
    if contents.is_err() {
        panic!("Couldn't read from {}", filepath);
    }
    contents.unwrap()
}

fn main() {
    // day1::solution();
    // day2::solution();
    day3::solution();
}
