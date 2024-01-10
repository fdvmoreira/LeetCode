use aoc_2023::day9_part_two::extrapolated_sum;
use aoc_2023::utils::load_file_content;

fn main() {
    let data = load_file_content("data/day9.txt").unwrap();
    println!("Calculating ...");
    let ways =
        extrapolated_sum(&data.iter().map(|row| row.as_str()).collect::<Vec<&str>>()).unwrap();
    println!("Result {:?}", ways);
}
