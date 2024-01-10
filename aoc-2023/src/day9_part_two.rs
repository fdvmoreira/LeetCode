// --- Part Two ---
// Of course, it would be nice to have even more history included in your report. Surely it's safe to just extrapolate backwards as well, right?
//
// For each history, repeat the process of finding differences until the sequence of differences is entirely zero. Then, rather than adding a zero to the end and filling in the next values of each previous sequence, you should instead add a zero to the beginning of your sequence of zeroes, then fill in new first values for each previous sequence.
//
// In particular, here is what the third example history looks like when extrapolating back in time:
//
// 5  10  13  16  21  30  45
//   5   3   3   5   9  15
//    -2   0   2   4   6
//       2   2   2   2
//         0   0   0
// Adding the new values on the left side of each sequence from bottom to top eventually reveals the new left-most history value: 5.
//
// Doing this for the remaining example data above results in previous values of -3 for the first history and 0 for the second history. Adding all three new values together produces 2.
//
// Analyze your OASIS report again, this time extrapolating the previous value for each history. What is the sum of these extrapolated values?
//
//

use std::collections::VecDeque;

fn extrapolate_prev_value(sequence: &[i32]) -> Option<i32> {
    if sequence.is_empty() {
        return None;
    }

    let mut stack = Vec::<VecDeque<i32>>::new();
    let mut curr_seq: VecDeque<i32> = sequence.to_vec().into();

    while curr_seq.iter().any(|n| *n != 0) {
        let mut tmp = VecDeque::<i32>::new();
        for idx in 1..curr_seq.len() {
            tmp.push_back(curr_seq[idx] - curr_seq[idx - 1]);
        }
        stack.push(tmp.to_owned());
        curr_seq = tmp;
    }

    stack.last_mut().unwrap().push_front(0);
    while stack.len() != 1 {
        let last_first = stack.pop().unwrap().iter().nth(0).unwrap().to_owned();
        let prev_seq = stack.last_mut().unwrap();
        prev_seq.push_front(prev_seq.iter().nth(0).unwrap() - last_first);
    }

    let prev_val = stack.iter().last().unwrap().iter().nth(0).unwrap();

    Some(sequence.first().unwrap() - *prev_val)
}

pub fn extrapolated_sum(data: &[&str]) -> Option<i32> {
    let parsed_data = data
        .to_vec()
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let sum = parsed_data
        .iter()
        .map(|v| extrapolate_prev_value(&v).unwrap())
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use googletest::{assert_that, matchers::eq};

    use super::*;

    #[test]
    fn test_extrapolate_prev_value() {
        let data = vec![0, 3, 6, 9, 12, 15];
        let output = extrapolate_prev_value(&data).unwrap_or_else(|| 0);
        assert_that!(output, eq(-3));
        let data = vec![1, 3, 6, 10, 15, 21];
        let output = extrapolate_prev_value(&data).unwrap_or_else(|| 0);
        assert_that!(output, eq(0));
        let data = vec![10, 13, 16, 21, 30, 45];
        let output = extrapolate_prev_value(&data).unwrap_or_else(|| 0);
        assert_that!(output, eq(5));
    }

    #[test]
    fn test_extrapolated_sum() {
        let data = ["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];
        let output = extrapolated_sum(&data).unwrap_or_else(|| 0);
        assert_that!(output, eq(2))
    }
}
