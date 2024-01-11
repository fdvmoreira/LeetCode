use aoc_2023::day10::total_steps;
use aoc_2023::utils::load_file_content;

fn main() {
    let data = load_file_content("data/day9.txt").unwrap();
    println!("Calculating ...");
    let steps = total_steps(&data.iter().map(|row| row.as_str()).collect::<Vec<&str>>()).unwrap();
    println!("Result {:?}", steps);
}
