use aoc_2023::day7_part_two::total_winnings;
use aoc_2023::utils::load_file_content;

fn main() {
    let data = load_file_content("data/day7.txt").unwrap();
    println!("Calculating ...");
    let ways = total_winnings(&data.iter().map(|row| row.as_str()).collect::<Vec<&str>>()).unwrap();
    println!("Result {:?}", ways);
}
