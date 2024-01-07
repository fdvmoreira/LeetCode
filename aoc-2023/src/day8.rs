// --- Day 8: Haunted Wasteland ---
// You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching. When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just finished warning you about ghosts a few minutes ago.
//
// One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle input) about how to navigate the desert. At least, you're pretty sure that's what they are; one of the documents contains a list of left/right instructions, and the rest of the documents seem to describe some kind of network of labeled nodes.
//
// It seems like you're meant to use the left/right instructions to navigate the network. Perhaps if you have the camel follow the same instructions, you can escape the haunted wasteland!
//
// After examining the maps for a bit, two nodes stick out: AAA and ZZZ. You feel like AAA is where you are now, and you have to follow the left/right instructions until you reach ZZZ.
//
// This format defines each node of the network individually. For example:
//
// RL
//
// AAA = (BBB, CCC)
// BBB = (DDD, EEE)
// CCC = (ZZZ, GGG)
// DDD = (DDD, DDD)
// EEE = (EEE, EEE)
// GGG = (GGG, GGG)
// ZZZ = (ZZZ, ZZZ)
// Starting with AAA, you need to look up the next element based on the next left/right instruction in your input. In this example, start with AAA and go right (R) by choosing the right element of AAA, CCC. Then, L means to choose the left element of CCC, ZZZ. By following the left/right instructions, you reach ZZZ in 2 steps.
//
// Of course, you might not find ZZZ right away. If you run out of left/right instructions, repeat the whole sequence of instructions as necessary: RL really means RLRLRLRLRLRLRLRL... and so on. For example, here is a situation that takes 6 steps to reach ZZZ:
//
// LLR
//
// AAA = (BBB, BBB)
// BBB = (AAA, ZZZ)
// ZZZ = (ZZZ, ZZZ)
// Starting at AAA, follow the left/right instructions. How many steps are required to reach ZZZ?
//
use std::collections::HashMap;

type ParsedDirNNetwork = Option<(Vec<char>, HashMap<String, (String, String)>)>;

pub fn parse_directions_and_network(data: &[&str]) -> ParsedDirNNetwork {
    let [directions, network] = &data.split(|line| line.is_empty()).collect::<Vec<_>>()[..] else {
        todo!()
    };

    let directions = directions
        .to_owned()
        .last()
        .unwrap()
        .to_string()
        .chars()
        .collect::<Vec<char>>();

    let network = network
        .iter()
        .map(|line| {
            let [key, val] = &line
                .to_owned()
                .split('=')
                .map(|v| v.trim().to_owned().to_string())
                .collect::<Vec<String>>()[..]
            else {
                todo!()
            };
            let [left, right] = &val
                .trim_matches([')', '('])
                .split_terminator(',')
                .map(|w| w.trim().to_string())
                .collect::<Vec<String>>()[..]
            else {
                todo!()
            };
            (key.to_string(), (left.to_string(), right.to_string()))
        })
        // .collect::<Vec<_>>();
        .collect::<Vec<(String, (String, String))>>();

    let network = HashMap::<String, (String, String)>::from_iter(network);

    Some((directions, network))
}

pub fn get_first_and_last_nodes(data: &[&str]) -> Option<(String, String)> {
    let snd_part = data.split(|line| line.is_empty()).last().unwrap();

    let [first, .., last] = snd_part else { todo!() };

    Some((
        first.split('=').nth(0).unwrap().trim().to_string(),
        last.split('=').nth(0).unwrap().trim().to_string(),
    ))
}

pub fn total_steps(data: &[&str]) -> Option<u32> {
    let (start, target) = get_first_and_last_nodes(&data).unwrap();
    let (directions, network) = parse_directions_and_network(&data).unwrap();

    let mut total_steps = 0;
    let mut curr_node = start;

    while curr_node != target {
        let node = network.get(&curr_node).unwrap();

        curr_node = if directions.get(total_steps % directions.len()).unwrap() == &'L' {
            node.0.to_owned()
        } else {
            node.1.to_owned()
        };

        total_steps += 1;
        println!(
            "Nlen:{} Tsteps:{} currN:{curr_node} Tgt:{target}",
            network.len(),
            total_steps
        );
    }

    Some(total_steps as u32)
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use googletest::{assert_that, matchers::eq};

    use super::*;
    #[test]
    fn test_total_steps() {
        let input = vec![
            "RL",
            "",
            "AAA = (BBB, CCC)",
            "BBB = (DDD, EEE)",
            "CCC = (ZZZ, GGG)",
            "DDD = (DDD, DDD)",
            "EEE = (EEE, EEE)",
            "GGG = (GGG, GGG)",
            "ZZZ = (ZZZ, ZZZ)",
        ];
        let result = total_steps(&input).unwrap();
        assert_that!(result, eq(2));

        let input = vec![
            "LLR",
            "",
            "AAA = (BBB, BBB)",
            "BBB = (AAA, ZZZ)",
            "ZZZ = (ZZZ, ZZZ)",
        ];
        let result = total_steps(&input).unwrap();
        assert_that!(result, eq(6));
    }

    #[googletest::test]
    fn test_parse_directions_and_network() {
        let data = vec![
            "LLR",
            "",
            "AAA = (BBB, BBB)",
            "BBB = (AAA, ZZZ)",
            "ZZZ = (ZZZ, ZZZ)",
        ];
        let result = parse_directions_and_network(&data).unwrap();
        let (directions, network) = (result.0, result.1);

        let expected_vec = vec!['L', 'L', 'R'];
        let expected_hashmap = HashMap::<String, (String, String)>::from([
            ("AAA".to_string(), ("BBB".to_string(), "BBB".to_string())),
            ("BBB".to_string(), ("AAA".to_string(), "ZZZ".to_string())),
            ("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string())),
        ]);

        assert_that!(directions, eq(expected_vec));
        assert_that!(network.len(), eq(expected_hashmap.len()));
        assert!(expected_hashmap.contains_key("AAA"));
        assert!(expected_hashmap.contains_key("BBB"));
        assert!(expected_hashmap.contains_key("ZZZ"));
    }

    #[test]
    fn test_get_first_and_last_nodes() {
        let data = vec![
            "LRLR",
            "",
            "AAA = (CCC,DDD)",
            "BBB = (FFF,JJJ)",
            "XXX = (RRR,EEE)",
        ];
        let output = get_first_and_last_nodes(&data).unwrap();

        assert_that!(output.0, eq("AAA".to_string()));
        assert_that!(output.1, eq("XXX".to_string()));
    }
}
