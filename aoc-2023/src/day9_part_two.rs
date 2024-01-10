fn extrapolate_prev_value(sequence: &[i32]) -> Option<i32> {
    if sequence.is_empty() {
        return None;
    }

    let mut stack = Vec::<Vec<i32>>::new();
    let mut curr_seq: Vec<i32> = sequence.to_vec();

    while curr_seq.iter().any(|n| *n != 0) {
        let mut tmp = vec![];
        for idx in 1..curr_seq.len() {
            tmp.push(curr_seq[idx] - curr_seq[idx - 1]);
        }
        stack.push(tmp.to_owned());
        curr_seq = tmp;
    }

    stack.last_mut().unwrap().push(0);

    while stack.len() != 1 {
        let last = stack.pop().unwrap().last().unwrap().to_owned();
        let prev_seq = stack.last_mut().unwrap();
        prev_seq.push(last + prev_seq.iter().last().unwrap());
    }

    let next_val = stack.iter().last().unwrap().iter().last().unwrap();

    Some(*next_val + sequence.last().unwrap())
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
        assert_that!(output, eq(-1));
        let data = vec![10, 13, 16, 21, 30, 45];
        let output = extrapolate_prev_value(&data).unwrap_or_else(|| 0);
        assert_that!(output, eq(5));
    }

    #[test]
    fn test_extrapolated_sum() {
        let data = ["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];
        let output = extrapolated_sum(&data).unwrap_or_else(|| 0);
        assert_that!(output, eq(114))
    }
}
