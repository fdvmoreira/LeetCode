pub mod day1 {
    use std::{
        char,
        fs::File,
        io::{BufRead, BufReader, Error},
        u32,
    };

    fn load_file_content(path: &str) -> Result<Vec<String>, Error> {
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

            let (first, last) = (digits.first().unwrap(), digits.last().unwrap());

            let num = format!("{}{}", first.unwrap(), last.unwrap())
                .parse::<u32>()
                .unwrap();

            acc + num
        });

        Some(sum)
    }
}
