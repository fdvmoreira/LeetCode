// pub mod day1;
// pub mod day2;
pub mod day3;

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

#[cfg(test)]
mod tests {
    use googletest::assert_pred;

    use crate::utils::load_file_content;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_load_file_content() {
        // Create a temporary file with some content for testing
        let path = "test_file.txt";
        let content = "Line 1\nLine 2\nLine 3";

        let mut file = File::create(path).expect("Failed to create test file");
        file.write_all(content.as_bytes())
            .expect("Failed to write to test file");

        // Test the load_file_content function
        match load_file_content(path) {
            Ok(lines) => {
                // Check if the content matches the lines read from the file
                assert_eq!(lines, vec!["Line 1", "Line 2", "Line 3"]);
            }
            Err(e) => {
                // Handle the error, you may want to print it or log it
                eprintln!("Error reading file: {}", e);
                assert!(false, "Test failed due to an error");
            }
        }

        // Clean up: delete the temporary file
        std::fs::remove_file(path).expect("Failed to remove test file");
    }

    #[test]
    fn test_load_file_content_failure() {
        let actual = load_file_content("yayayay");
        assert_pred!(actual.is_err());
    }
}
