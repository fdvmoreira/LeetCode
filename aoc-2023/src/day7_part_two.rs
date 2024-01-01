// --- Part Two ---
// To make things a little more interesting, the Elf introduces one additional rule. Now, J cards are jokers - wildcards that can act like whatever card would make the hand the strongest type possible.
//
// To balance this, J cards are now the weakest individual cards, weaker even than 2. The other cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.
//
// J cards can pretend to be whatever card is best for the purpose of determining hand type; for example, QJJQ2 is now considered four of a kind. However, for the purpose of breaking ties between two hands of the same type, J is always treated as J, not the card it's pretending to be: JKKK2 is weaker than QQQQ2 because J is weaker than Q.
//
// Now, the above example goes very differently:
//
// 32T3K 765
// T55J5 684
// KK677 28
// KTJJT 220
// QQQJA 483
// 32T3K is still the only one pair; it doesn't contain any jokers, so its strength doesn't increase.
// KK677 is now the only two pair, making it the second-weakest hand.
// T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3, QQQJA gets rank 4, and KTJJT gets rank 5.
// With the new joker rule, the total winnings in this example are 5905.
//
// Using the new joker rule, find the rank of every hand in your set. What are the new total winnings?
//
//

use std::{cmp::Ordering, collections::HashMap, usize};

#[repr(u8)]
#[derive(Debug)]
pub enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn get_hand_type(hand: &str) -> Option<HandType> {
    if hand.len() != 5 {
        return None;
    }

    let mut chars: HashMap<String, u32> = HashMap::new();
    hand.chars().for_each(|char| {
        chars.insert(
            char.to_string(),
            chars.get(&char.to_string()).unwrap_or(&0) + 1,
        );
    });

    if let Some(joker_count) = chars.get("J") {
        let (mut current_max_k, mut current_max_v) = ("".to_string(), 0u32);
        for (k, v) in chars.iter().filter(|(k, _)| **k != "J".to_string()) {
            if *v > current_max_v {
                current_max_v = v.clone();
                current_max_k = k.clone();
            }
        }
        chars.insert(current_max_k, current_max_v + joker_count);
        chars.remove_entry("J");
    }

    if chars.len() == 5 {
        return Some(HandType::HighCard);
    } else if chars.len() == 4 {
        return Some(HandType::OnePair);
    } else if chars.len() == 3 && chars.values().any(|v| *v == 2) {
        return Some(HandType::TwoPair);
    } else if chars.len() == 3 && chars.values().any(|v| *v == 3) {
        return Some(HandType::ThreeOfAKind);
    } else if chars.len() == 2 && chars.values().any(|v| *v == 3) {
        return Some(HandType::FullHouse);
    } else if chars.len() == 2 && chars.values().any(|v| *v == 4) {
        return Some(HandType::FourOfAKind);
    } else if chars.len() == 1 {
        return Some(HandType::FiveOfAKind);
    } else {
        return None;
    }
}

pub fn total_winnings(data: &[&str]) -> Option<u32> {
    if data.is_empty() {
        return None;
    }

    let card_map: HashMap<&str, usize> = HashMap::from_iter(
        vec![
            "A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2", "J",
        ]
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, s)| (*s, idx)),
    );

    let mut hands = data
        .iter()
        .map(|line| {
            let biding = line.split_ascii_whitespace().collect::<Vec<_>>();
            let [hand, bid] = biding.as_slice() else {
                todo!()
            };
            let hand_type = get_hand_type(hand).unwrap() as u32;
            (hand_type, *hand, *bid)
        })
        .collect::<Vec<(u32, &str, &str)>>();

    // sort and reverse the hands
    hands.sort_by(|a, b| {
        let mut ord = a.0.cmp(&b.0);

        let mut idx = 0;
        while ord == Ordering::Equal && idx < a.1.len() {
            ord = card_map
                .get(a.1.get(idx..idx + 1).unwrap())
                .unwrap()
                .cmp(card_map.get(b.1.get(idx..idx + 1).unwrap()).unwrap());
            idx += 1;
        }

        ord
    });

    // rank and sum
    let total = hands.iter().enumerate().fold(0, |acc, tpl| {
        acc + (tpl.0 as u32 + 1) * (tpl.1).2.parse::<u32>().unwrap()
    });
    Some(total)
}

#[cfg(test)]
mod tests {
    use googletest::{assert_that, matchers::eq};

    use super::*;

    #[test]
    fn test_total_winnings() {
        let input = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ];
        let result = total_winnings(&input).unwrap();
        let expected = 5905;
        assert_that!(result, eq(expected));
    }

    #[test]
    fn test_get_hand_type() {
        let hand = "AAAAA";
        let result = get_hand_type(hand).unwrap() as u32;
        assert_that!(result, eq(HandType::FiveOfAKind as u32));

        let hand = "T55J5";
        let result = get_hand_type(hand).unwrap() as u32;
        assert_that!(result, eq(HandType::FourOfAKind as u32));

        let hand = "KTJJT";
        let result = get_hand_type(hand).unwrap() as u32;
        assert_that!(result, eq(HandType::FourOfAKind as u32));

        let hand = "QQQJA";
        let result = get_hand_type(hand).unwrap() as u32;
        assert_that!(result, eq(HandType::FourOfAKind as u32));

        let hand = "AA88A";
        let result = get_hand_type(hand).unwrap() as u32;
        assert_that!(result, eq(HandType::FullHouse as u32));

        let hand = "AA28A";
        let result = get_hand_type(hand).unwrap() as u32;
        assert_that!(result, eq(HandType::ThreeOfAKind as u32));

        let hand = "KK677";
        let result = get_hand_type(hand).unwrap() as u32;
        assert_that!(result, eq(HandType::TwoPair as u32));

        let hand = "32T3K";
        let result = get_hand_type(hand).unwrap() as u32;
        assert_that!(result, eq(HandType::OnePair as u32));

        let hand = "2358A";
        let result = get_hand_type(hand).unwrap() as u32;
        assert_that!(result, eq(HandType::HighCard as u32));
    }
}
