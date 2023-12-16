use aoc_2023::day4::calculate_total_points;
use aoc_2023::utils::load_file_content;

fn main() {
    let data = load_file_content("data/day4.txt").unwrap();
    let total_points =
        calculate_total_points(&data.iter().map(|row| row.as_str()).collect::<Vec<&str>>())
            .unwrap();
    println!("Result {:?}", total_points);
}
