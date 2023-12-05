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
        let digits: Vec<char> = line.chars().filter(|b| b.is_ascii_digit()).collect();

        if !digits.is_empty() {
            let (first, last) = (digits.first().unwrap(), digits.last().unwrap());

            let num = format!("{}{}", first, last).parse::<u32>().unwrap();

            return acc + num;
        }
        acc
    });

    Some(sum)
}

// part two
pub fn trubuchet_two(path: &str) -> Option<u32> {
    let mut lines = load_file_content(path).ok()?;

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

    // replace the words with numbers
    let modified_lines: Vec<String> = lines
        .iter()
        .map(|line| replace_word_with_digit(line, &digit_names))
        .collect();
    // });

    let sum = modified_lines.iter().fold(0, |acc, line| {
        let digits: Vec<char> = line.chars().filter(|b| b.is_ascii_digit()).collect();

        if !digits.is_empty() {
            let (first, last) = (digits.first().unwrap(), digits.last().unwrap());

            let num = format!("{}{}", first, last).parse::<u32>().unwrap();

            return acc + num;
        }
        acc
    });

    Some(sum)
}

// REVIEW: use for loops instead of functional style
pub fn replace_word_with_digit(string: &str, vec: &HashMap<&str, u32>) -> String {
    let mut string = String::from(string);
    for key in vec.keys() {
        while string.contains(key) {
            let ki = string.find(key).unwrap_or_default();
            string.replace_range(
                ki..ki + key.len(),
                vec.get(key).unwrap().to_owned().to_string().as_str(),
            );
            println!("{string}");
        }
        // println!("{string}");
    }

    string
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use super::{load_file_content, replace_word_with_digit, trubuchet_one, trubuchet_two};
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

    #[googletest::test]
    fn should_conver_words_to_digits() {
        let digits: HashMap<&str, u32> = HashMap::from([
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

        let output = String::from("1223455667899");
        expect_that!(
            replace_word_with_digit(
                "onetwotwothreefourfivefivesixsixseveneightninenine",
                &digits
            ),
            eq(output)
        );
        let output2 = String::from("3944jclspd152rp");
        expect_that!(
            replace_word_with_digit("3nine44jclspd152rp", &digits),
            eq(output2)
        );
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
