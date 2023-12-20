use aoc_2023::day5::get_lowest_location;
use aoc_2023::utils::load_file_content;

fn main() {
    let data = load_file_content("data/day5.txt").unwrap();
    println!("Calculating ...");
    let minimum_location =
        get_lowest_location(&data.iter().map(|row| row.as_str()).collect::<Vec<&str>>()).unwrap();
    println!("Result {:?}", minimum_location);
}
