use aoc_2023::day3::{get_part_numbers, sum_part_numbers};
use aoc_2023::utils::load_file_content;

fn main() {
    let data = load_file_content("data/day3.txt").unwrap();
    let part_numbers =
        get_part_numbers(&data.iter().map(|row| row.as_str()).collect::<Vec<&str>>())
            .ok()
            .unwrap();
    let sum = sum_part_numbers(&part_numbers[..]).ok().unwrap();
    println!("Result {:?}", sum);
}
