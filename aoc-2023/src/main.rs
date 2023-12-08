use aoc_2023::day2::{get_possible_games_id_sum, parse_game_log, utils::load_file_content};

fn main() {
    let content = load_file_content("data/day2.txt");
    let games = parse_game_log(content.ok().unwrap()).ok();
    let output = get_possible_games_id_sum(games.unwrap());
    println!("Result {}", output.ok().unwrap());
}
