use aoc_2023::day6::max_winning_ways;
use aoc_2023::utils::load_file_content;

fn main() {
    let data = load_file_content("data/day6.txt").unwrap();
    println!("Calculating ...");
    let ways =
        max_winning_ways(&data.iter().map(|row| row.as_str()).collect::<Vec<&str>>()).unwrap();
    println!("Result {:?}", ways);
}
