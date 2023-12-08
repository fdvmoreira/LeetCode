//
// --- Day 2: Cube Conundrum ---
// You're launched high into the atmosphere! The apex of your trajectory just barely reaches the surface of a large island floating in the sky. You gently land in a fluffy pile of leaves. It's quite cold, but you don't see much snow. An Elf runs over to greet you.
//
// The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow. He'll be happy to explain the situation, but it's a bit of a walk, so you have some time. They don't get many visitors up here; would you like to play a game in the meantime?
//
// As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue. Each time you play this game, he will hide a secret number of cubes of each color in the bag, and your goal is to figure out information about the number of cubes.
//
// To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few times per game.
//
// You play several games and record the information from each game (your puzzle input). Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).
//
// For example, the record of a few games might look like this:
//
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
// In game 1, three sets of cubes are revealed from the bag (and then put back again). The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.
//
// The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
//
// In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with that configuration. However, game 3 would have been impossible because at one point the Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible because the Elf showed you 15 blue cubes at once. If you add up the IDs of the games that would have been possible, you get 8.
//
// Determine which games would have been possible if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
// ---

use std::{collections::HashMap, io::Error};

pub mod utils {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    pub fn load_file_content(path: &str) -> Result<Vec<String>, std::io::Error> {
        let file = File::open(path)?;

        let reader = BufReader::new(file);

        let mut lines: Vec<String> = Vec::new();
        for line in reader.lines() {
            lines.push(line.unwrap());
        }

        Ok(lines)
    }
}

#[derive(Debug, PartialEq)]
pub struct Game {
    id: u32,
    records: Record,
}

#[derive(Debug, PartialEq)]
pub struct Record {
    reds: Vec<u32>,
    greens: Vec<u32>,
    blues: Vec<u32>,
}

pub fn get_possible_games_id_sum(games: Vec<Game>) -> Result<u32, Error> {
    const COLORS: (u32, u32, u32) = (12, 13, 14);

    let sum: u32 = games
        .iter()
        .filter(|&game| {
            let arr: [bool; 3] = [
                game.records.reds.iter().any(|count| count > &COLORS.0),
                game.records.greens.iter().any(|count| count > &COLORS.1),
                game.records.blues.iter().any(|count| count > &COLORS.2),
            ];

            arr.iter().all(|&v| !v)
        })
        .map(|game| game.id)
        .sum();

    Ok(sum)
}

pub fn parse_game_log(log: Vec<String>) -> Result<Vec<Game>, std::io::Error> {
    if log.is_empty() {
        panic!("log is empty")
    }

    let mut games: Vec<Game> = Vec::new();

    for game in log.iter() {
        let red: Vec<u32> = Vec::new();
        let green: Vec<u32> = Vec::new();
        let blue: Vec<u32> = Vec::new();
        let mut map: HashMap<&str, Vec<u32>> =
            HashMap::from([("red", red), ("green", green), ("blue", blue)]);

        let (id, record) = game.split_once(':').expect("failed extracting id");
        let id = id.split_whitespace().last();

        let rounds: Vec<&str> = record.split(';').collect();

        for round in rounds.iter() {
            let cubes: Vec<&str> = round.split(',').collect();

            for cube in cubes.iter() {
                let [num, color] = cube.split_ascii_whitespace().collect::<Vec<&str>>()[..] else {
                    todo!()
                };
                map.get_mut(color).unwrap().push(num.parse().unwrap());
            }
        }

        games.push(Game {
            id: id.unwrap().parse().unwrap(),
            records: Record {
                reds: map.get("red").unwrap().to_vec(),
                greens: map.get("green").unwrap().to_vec(),
                blues: map.get("blue").unwrap().to_vec(),
            },
        });
    }

    Ok(games)
}

#[cfg(test)]
mod tests {

    use super::utils::load_file_content;
    use super::*;
    use googletest::{
        assert_that, fail,
        matchers::{eq, err, gt, len},
        verify_that, Result,
    };

    #[googletest::test]
    fn should_fail_load_file() -> Result<()> {
        assert_that!(load_file_content("data/day2.txt"), len(gt(0)));
        let _ = verify_that!(load_file_content(""), len(eq(0)));
        Ok(())
    }

    #[googletest::test]
    fn test_get_possible_games_id_sum() -> Result<()> {
        // Test case where no game has counts greater than COLORS
        let games_no_match = vec![
            Game {
                id: 1,
                records: Record {
                    reds: vec![10, 11],
                    greens: vec![9, 12],
                    blues: vec![11, 13],
                },
            },
            Game {
                id: 2,
                records: Record {
                    reds: vec![8, 10],
                    greens: vec![10, 11],
                    blues: vec![9, 13],
                },
            },
            Game {
                id: 3,
                records: Record {
                    reds: vec![3, 4, 5],
                    greens: vec![14, 3, 6],
                    blues: vec![3],
                },
            },
        ];
        assert_that!(get_possible_games_id_sum(games_no_match).unwrap(), eq(3));

        // Test case where one game has counts greater than COLORS
        let games_one_match = vec![
            Game {
                id: 1,
                records: Record {
                    reds: vec![13, 11],
                    greens: vec![9, 12],
                    blues: vec![11, 13],
                },
            },
            Game {
                id: 2,
                records: Record {
                    reds: vec![8, 10],
                    greens: vec![10, 11],
                    blues: vec![9, 13],
                },
            },
        ];
        assert_that!(get_possible_games_id_sum(games_one_match).unwrap(), eq(2));

        // Test case where all games have counts greater than COLORS
        let games_all_match = vec![
            Game {
                id: 1,
                records: Record {
                    reds: vec![15, 16],
                    greens: vec![17, 18],
                    blues: vec![14, 15],
                },
            },
            Game {
                id: 2,
                records: Record {
                    reds: vec![12, 13],
                    greens: vec![13, 14],
                    blues: vec![11, 14],
                },
            },
        ];
        assert_that!(get_possible_games_id_sum(games_all_match).unwrap(), eq(0));

        // Fail the case
        let result = get_possible_games_id_sum(Vec::new()).unwrap();
        let _ = verify_that!(result, eq(0));
        Ok(())
    }

    #[test]
    fn test_parse_game_log_valid() {
        let log = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
        ];

        let expected_result = vec![
            Game {
                id: 1,
                records: Record {
                    reds: vec![4, 1],
                    greens: vec![2, 2],
                    blues: vec![3, 6],
                },
            },
            Game {
                id: 2,
                records: Record {
                    reds: vec![1],
                    greens: vec![2, 3, 1],
                    blues: vec![1, 4, 1],
                },
            },
        ];

        assert_that!(parse_game_log(log).unwrap(), eq(expected_result));
    }

    #[test]
    #[should_panic]
    fn test_parse_game_log_empty_log() -> () {
        let log: Vec<String> = vec![];
        let _result = parse_game_log(log);

        ()
    }
}
