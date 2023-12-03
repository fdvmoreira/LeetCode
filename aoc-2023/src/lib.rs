pub mod day1 {
    use std::{
        char,
        fs::File,
        io::{BufRead, BufReader, Error},
        u32,
    };

    pub fn load_file_content(path: &str) -> Result<Vec<String>, Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut lines = Vec::new();
        for line in reader.lines() {
            lines.push(line.unwrap_or_default());
        }

        Ok(lines)
    }

    pub fn trubuchet(path: &str) -> Option<u32> {
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
}

#[cfg(test)]
mod tests {

    use crate::day1::{load_file_content, trubuchet};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_load_file_content() {
        assert!(!load_file_content("data/day1.txt").unwrap().is_empty())
    }

    #[test]
    fn test_trubuchet() {
        assert_eq!(trubuchet("data/test/day1_01.txt"), Some(131u32));
        assert_eq!(trubuchet("data/test/day1_02.txt"), Some(22u32));
        assert_eq!(trubuchet("data/test/day1_03.txt"), Some(22u32));
    }
}
