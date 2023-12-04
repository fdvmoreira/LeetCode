use std::{
    char,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error},
    u32,
};

pub fn load_file_content(path: &str) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap_or_default().to_lowercase());
    }

    Ok(lines)
}

pub fn trubuchet_one(path: &str) -> Option<u32> {
    let lines = load_file_content(path).unwrap();

    if lines.is_empty() {
        return None;
    }

    let sum = lines.iter().fold(0, |acc, line| {
        let digits: Vec<Option<char>> = line
            .bytes()
            .filter(|b| b.is_ascii_digit())
            .map(|v| char::from_u32(v as u32))
            .collect();

        if !digits.is_empty() {
            let (first, last) = (digits.first().unwrap(), digits.last().unwrap());

            let num = format!("{}{}", first.unwrap(), last.unwrap())
                .parse::<u32>()
                .unwrap();

            return acc + num;
        }
        acc
    });

    Some(sum)
}

// part two
pub fn trubuchet_two(path: &str) -> Option<u32> {
    let mut lines = load_file_content(path).unwrap();

    let digit_names: HashMap<&str, u32> = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    if lines.is_empty() {
        return None;
    }

    // replace the words with the number
    lines.iter_mut().for_each(|line| {
        let mut line_copy = line.clone();

        for ele in digit_names.keys() {
            while line_copy.contains(ele) {
                line_copy.replace_range(
                    line_copy.find(ele).unwrap()..line_copy.find(ele).unwrap() + ele.len(),
                    digit_names
                        .get_key_value(ele)
                        .unwrap()
                        .1
                        .to_string()
                        .as_str(),
                ); //TODO - mutability problems
            }
        }

        line.replace_range(.., line_copy.as_str());
    });

    let sum = lines.iter().fold(0, |acc, line| {
        let digits: Vec<Option<char>> = line
            .bytes()
            .filter(|b| b.is_ascii_digit())
            .map(|v| char::from_u32(v as u32))
            .collect();

        if !digits.is_empty() {
            let (first, last) = (digits.first().unwrap(), digits.last().unwrap());

            let num = format!("{}{}", first.unwrap(), last.unwrap())
                .parse::<u32>()
                .unwrap();

            return acc + num;
        }
        acc
    });

    Some(sum)
}

#[cfg(test)]
mod tests {

    use super::{load_file_content, trubuchet_one, trubuchet_two};
    use googletest::prelude::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_load_file_content() {
        assert!(!load_file_content("data/day1.txt").unwrap().is_empty())
    }

    #[googletest::test]
    fn should_load_content_of_file() {
        expect_that!(load_file_content("data/day1.txt").unwrap(), not(empty()));
    }

    #[test]
    fn test_trubuchet_one() {
        assert_eq!(trubuchet_one("data/test/day1_01.txt"), Some(131u32));
        assert_eq!(trubuchet_one("data/test/day1_02.txt"), Some(22u32));
        assert_eq!(trubuchet_one("data/test/day1_03.txt"), Some(22u32));
    }

    #[test]
    fn test_trubuchet_two() {
        assert_eq!(trubuchet_two("data/test/day1_01.txt"), Some(140u32));
        assert_eq!(trubuchet_two("data/test/day1_02.txt"), Some(28u32));
        assert_eq!(trubuchet_two("data/test/day1_03.txt"), Some(55u32));
    }
}
