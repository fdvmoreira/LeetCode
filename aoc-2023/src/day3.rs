//
// --- Day 3: Gear Ratios ---
// You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.
//
// It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.
//
// "Aaah!"
//
// You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.
//
// The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.
//
// The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)
//
// Here is an example engine schematic:
//
// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
// In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.
//
// Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?
//
//
//

// ditermine part number
// if surrounding values are diff from dot(.) and another number

use std::{char, collections::HashSet, str::FromStr, usize};

pub fn is_part_number(chars: &[char]) -> Result<bool, std::io::ErrorKind> {
    if chars.is_empty() {
        return Ok(false);
    }

    let nums: HashSet<char> =
        HashSet::from(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.']);

    Ok(chars.iter().any(|c| (!nums.contains(c))).to_owned())
}

pub fn get_part_numbers(data: &[&str]) -> Result<Vec<u32>, std::io::ErrorKind> {
    if data.is_empty() {
        return Err(std::io::ErrorKind::InvalidInput);
    }

    let matrix: Vec<Vec<char>> = data
        .into_iter()
        .map(|v| v.chars().collect::<Vec<char>>())
        .collect();

    let mut part_numbers: Vec<u32> = Vec::new();

    for (ridx, row) in matrix.iter().enumerate() {
        for mut cidx in 0..row.len() {
            let mut is_curr_val_num = (matrix[ridx][cidx]).is_digit(10);

            if is_curr_val_num {
                let start_idx = cidx;
                let mut end_idx = cidx;
                let mut current_num = String::new();

                while is_curr_val_num {
                    current_num.push(matrix[ridx][cidx]);
                    cidx += 1;
                    is_curr_val_num = (matrix[ridx][cidx]).is_digit(10);
                    end_idx = cidx;
                }

                let up = matrix[ridx - 1].get(start_idx - 1..=end_idx);
                let down = matrix[ridx + 1].get(start_idx - 1..=end_idx);
                let back = matrix[ridx].get(start_idx - 1..start_idx - 1);
                let front = matrix[ridx].get(end_idx + 1..end_idx + 1);

                // if let Some(_) = is_part_number(
                //     &[up.into(), down.into(), back.into(), front.into()][..]
                //         .iter_mut()
                //         .flatten()
                //         .collect(),
                // )
                // .ok()
                // {
                //     part_numbers.push(current_num.parse::<u32>().unwrap_or(0));
                // }
            }
        }
    }

    Ok(part_numbers)
}

pub fn sum_part_numbers(part_numbers: &[u32]) -> Result<u32, std::io::ErrorKind> {
    if part_numbers.is_empty() {
        return Err(std::io::ErrorKind::InvalidInput);
    }

    let sum: u32 = part_numbers.iter().sum::<u32>();

    Ok(sum)
}

pub mod utils {}

#[cfg(test)]
mod tests {

    use super::*;
    use googletest::{assert_that, matchers::eq};

    #[test]
    fn test_empty_array() {
        let nums: Vec<u32> = vec![];

        assert_that!(
            sum_part_numbers(&nums),
            eq(Err(std::io::ErrorKind::InvalidInput))
        );
    }

    #[test]
    fn test_single_element_array() {
        let nums: Vec<u32> = vec![1];

        assert!(sum_part_numbers(&nums).is_ok());
        let sum = sum_part_numbers(&nums).unwrap();
        assert_that!(sum, eq(1));
    }

    #[test]
    fn test_array_with_unique_elements() {
        let nums: Vec<u32> = vec![1, 2, 3, 4];

        assert!(sum_part_numbers(&nums).is_ok());
        let sum = sum_part_numbers(&nums).unwrap();
        assert_that!(sum, eq(10));
    }

    #[test]
    fn test_array_with_duplicate_elements() {
        let nums: Vec<u32> = vec![1, 1, 2, 2, 3, 3, 4];

        assert!(sum_part_numbers(&nums).is_ok());
        let sum = sum_part_numbers(&nums).unwrap();
        assert_that!(sum, eq(16));
    }

    #[test]
    fn test_is_part_number_valid() {
        // Valid part number: contains only digits and dot
        let input = &['1', '2', '3', '.', '*', '5'];
        let result = is_part_number(input).unwrap();
        assert!(result);
    }

    #[test]
    fn test_is_part_number_invalid() {
        // Invalid part number: contains a character other than digit or dot
        let input = &['1', '2', '9', '4', '5'];
        let result = is_part_number(input).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_is_part_number_empty() {
        // Empty input should be considered invalid
        let input: &[char] = &[];
        let result = is_part_number(input).unwrap();
        assert!(!result);
    }
}
