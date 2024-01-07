use aoc_2023::day8::total_steps;
use aoc_2023::utils::load_file_content;

fn main() {
    let data = load_file_content("data/day8.txt").unwrap();
    println!("Calculating ...");
    let ways = total_steps(&data.iter().map(|row| row.as_str()).collect::<Vec<&str>>()).unwrap();
    println!("Result {:?}", ways);
}
