use aoc_2023::day10::furthest_point_from_start;
use aoc_2023::utils::load_file_content;

fn main() {
    let data = load_file_content("data/day10.txt").unwrap();
    println!("Calculating ...");
    let steps =
        furthest_point_from_start(&data.iter().map(|row| row.as_str()).collect::<Vec<&str>>())
            .unwrap();
    println!("Result {:?}", steps);
}
