// --- Part Two ---
// As the race is about to start, you realize the piece of paper with race times and record distances you got earlier actually just has very bad kerning. There's really only one race - ignore the spaces between the numbers on each line.
//
// So, the example from before:
//
// Time:      7  15   30
// Distance:  9  40  200
// ...now instead means this:
//
// Time:      71530
// Distance:  940200
// Now, you have to figure out how many ways there are to win this single race. In this example, the race lasts for 71530 milliseconds and the record distance you need to beat is 940200 millimeters. You could hold the button anywhere from 14 to 71516 milliseconds and beat the record, a total of 71503 ways!
//
// How many ways can you beat the record in this one much longer race?
//

pub fn max_winning_ways(races: &[&str]) -> Option<u32> {
    let Some(races) = parse_races(&races) else {
        return None;
    };

    let mut ways: u32 = 0;

    races.iter().for_each(|(time, record_distance)| {
        let mut new_record_counter = 0;
        for hold_dur in 0..=*time {
            let new_distances = hold_dur * (time - hold_dur);
            if new_distances > *record_distance {
                new_record_counter += 1;
            }
        }
        if ways == 0 {
            ways = new_record_counter;
            return;
        }

        ways *= new_record_counter;
    });

    Some(ways)
}

pub fn parse_races(races: &[&str]) -> Option<Vec<(u32, u32)>> {
    if races.is_empty() {
        return None;
    }

    let [times, distances] = &races
        .into_iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()[..]
    else {
        todo!()
    };

    let result = times
        .iter()
        .zip(distances.iter())
        .into_iter()
        .map(|(fst, snd)| (*fst, *snd))
        .collect::<Vec<(u32, u32)>>();

    Some(result)
}

#[cfg(test)]
mod tests {
    use googletest::{assert_that, matchers::eq};

    use super::*;

    #[test]
    fn test_parse_races() {
        let data = vec!["Time:      7  15   30", "Distance:  9  40  200"];
        let result = parse_races(&data).unwrap();
        assert_that!(result, eq(vec![(7, 9), (15, 40), (30, 200)]));

        let data = vec![];
        let result = parse_races(&data);
        assert!(result.is_none());
    }

    #[test]
    fn test_max_winning_ways() {
        let data = vec!["Time:      7  15   30", "Distance:  9  40  200"];
        let result = max_winning_ways(&data).unwrap();
        assert_that!(result, eq(288));

        let data = vec![];
        let result = max_winning_ways(&data);
        assert!(result.is_none());
    }
}
