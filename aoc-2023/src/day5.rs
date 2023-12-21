// --- Day 5: If You Give A Seed A Fertilizer ---
// You take the boat and find the gardener right where you were told he would be: managing a giant "garden" that looks more to you like a farm.
//
// "A water source? Island Island is the water source!" You point out that Snow Island isn't receiving any water.
//
// "Oh, we had to stop the water because we ran out of sand to filter it with! Can't make snow with dirty water. Don't worry, I'm sure we'll get more sand soon; we only turned off the water a few days... weeks... oh no." His face sinks into a look of horrified realization.
//
// "I've been so busy making sure everyone here has food that I completely forgot to check why we stopped getting more sand! There's a ferry leaving soon that is headed over in that direction - it's much faster than your boat. Could you please go check it out?"
//
// You barely have time to agree to this request when he brings up another. "While you wait for the ferry, maybe you can help us with our food production problem. The latest Island Island Almanac just arrived and we're having trouble making sense of it."
//
// The almanac (your puzzle input) lists all of the seeds that need to be planted. It also lists what type of soil to use with each kind of seed, what type of fertilizer to use with each kind of soil, what type of water to use with each kind of fertilizer, and so on. Every type of seed, soil, fertilizer and so on is identified with a number, but numbers are reused by each category - that is, soil 123 and fertilizer 123 aren't necessarily related to each other.
//
// For example:
//
// seeds: 79 14 55 13
//
// seed-to-soil map:
// 50 98 2
// 52 50 48
//
// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15
//
// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4
//
// water-to-light map:
// 88 18 7
// 18 25 70
//
// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13
//
// temperature-to-humidity map:
// 0 69 1
// 1 0 69
//
// humidity-to-location map:
// 60 56 37
// 56 93 4
// The almanac starts by listing which seeds need to be planted: seeds 79, 14, 55, and 13.
//
// The rest of the almanac contains a list of maps which describe how to convert numbers from a source category into numbers in a destination category. That is, the section that starts with seed-to-soil map: describes how to convert a seed number (the source) to a soil number (the destination). This lets the gardener and his team know which soil to use with which seeds, which water to use with which fertilizer, and so on.
//
// Rather than list every source number and its corresponding destination number one by one, the maps describe entire ranges of numbers that can be converted. Each line within a map contains three numbers: the destination range start, the source range start, and the range length.
//
// Consider again the example seed-to-soil map:
//
// 50 98 2
// 52 50 48
// The first line has a destination range start of 50, a source range start of 98, and a range length of 2. This line means that the source range starts at 98 and contains two values: 98 and 99. The destination range is the same length, but it starts at 50, so its two values are 50 and 51. With this information, you know that seed number 98 corresponds to soil number 50 and that seed number 99 corresponds to soil number 51.
//
// The second line means that the source range starts at 50 and contains 48 values: 50, 51, ..., 96, 97. This corresponds to a destination range starting at 52 and also containing 48 values: 52, 53, ..., 98, 99. So, seed number 53 corresponds to soil number 55.
//
// Any source numbers that aren't mapped correspond to the same destination number. So, seed number 10 corresponds to soil number 10.
//
// So, the entire list of seed numbers and their corresponding soil numbers looks like this:
//
// seed  soil
// 0     0
// 1     1
// ...   ...
// 48    48
// 49    49
// 50    52
// 51    53
// ...   ...
// 96    98
// 97    99
// 98    50
// 99    51
// With this map, you can look up the soil number required for each initial seed number:
//
// Seed number 79 corresponds to soil number 81.
// Seed number 14 corresponds to soil number 14.
// Seed number 55 corresponds to soil number 57.
// Seed number 13 corresponds to soil number 13.
// The gardener and his team want to get started as soon as possible, so they'd like to know the closest location that needs a seed. Using these maps, find the lowest location number that corresponds to any of the initial seeds. To do this, you'll need to convert each seed number through other categories until you can find its corresponding location number. In this example, the corresponding types are:
//
// Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
// Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
// Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
// Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.
// So, the lowest location number in this example is 35.
//
// What is the lowest location number that corresponds to any of the initial seed numbers?
//
//

use std::{collections::HashMap, u32, u64};

fn parse_seeds(seeds: &[&str]) -> Option<Vec<u32>> {
    if seeds.is_empty() {
        return None;
    }

    let seeds: Vec<u32> = seeds
        .last()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    Some(seeds)
}

fn map_src_to_dst(input: &[&str]) -> Option<HashMap<u32, u32>> {
    if input.is_empty() {
        return None;
    }

    let mut output: HashMap<u32, u32> = HashMap::new();

    input
        .iter()
        .skip(1)
        .map(|v| v.split_ascii_whitespace())
        .map(|v| v.map(|d| d.parse::<u32>().unwrap()).collect::<Vec<u32>>())
        .for_each(|vec| {
            let [dst, src, len] = vec.as_slice() else {
                todo!()
            };

            for i in 0..*len {
                output.insert(*src + i, dst + i);
            }
        });

    Some(output)
}

fn get_dst_value(target: &u64, haystack: &[&str]) -> Option<u64> {
    if haystack.len() < 2 {
        return None;
    }

    let range = haystack
        .iter()
        .skip(1)
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|val| val.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>()
        .into_iter()
        .filter(|vec| {
            let [_dst, src, len] = &vec[..] else { todo!() };

            target >= src && target <= &(src + len)
        })
        .last()
        .unwrap_or_else(|| vec![*target]);

    if range.len() < 3 {
        return Some(*target as u64);
    }

    let result: u64 = (range[0] + (target - range[1])) as u64;

    Some(result)
}

pub fn get_lowest_location(data: &[&str]) -> Result<u32, std::io::ErrorKind> {
    let [seeds, seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location] =
        data.split(|line| line.is_empty()).collect::<Vec<_>>()[..]
    else {
        todo!()
    };

    let seeds: Vec<u32> = parse_seeds(seeds).unwrap();

    let lowest_location = seeds
        .into_iter()
        .map(|seed| {
            get_dst_value(&(seed as u64), seed_to_soil).map_or_else(
                || 0,
                |soil| {
                    get_dst_value(&soil, soil_to_fertilizer).map_or_else(
                        || 0,
                        |fertilizer| {
                            get_dst_value(&fertilizer, fertilizer_to_water).map_or_else(
                                || 0,
                                |water| {
                                    get_dst_value(&water, water_to_light).map_or_else(
                                        || 0,
                                        |light| {
                                            get_dst_value(&light, light_to_temperature).map_or_else(
                                                || 0,
                                                |temp| {
                                                    get_dst_value(&temp, temperature_to_humidity)
                                                        .map_or_else(
                                                            || 0,
                                                            |humidity| {
                                                                get_dst_value(
                                                                    &humidity,
                                                                    humidity_to_location,
                                                                )
                                                                .unwrap()
                                                            },
                                                        )
                                                },
                                            )
                                        },
                                    )
                                },
                            )
                        },
                    )
                },
            )
        })
        .min();

    Ok((lowest_location.unwrap() as u64).try_into().unwrap())
}

pub mod utils {
    pub fn parse_file() {}
}

#[cfg(test)]
mod tests {

    use crate::utils::load_file_content;

    use super::*;
    use googletest::{
        assert_pred, assert_that,
        matchers::{eq, some},
        test,
    };

    #[test]
    fn test_get_lowest_location() {
        let data = load_file_content("data/test/day5.txt").ok().unwrap();
        let result = get_lowest_location(
            data.iter()
                .map(|line| line.as_str())
                .collect::<Vec<&str>>()
                .as_slice(),
        )
        .unwrap();
        assert_that!(result, eq(35));
    }

    #[googletest::test]
    fn test_parse_seeds() -> googletest::Result<()> {
        let input = [];
        let result = parse_seeds(&input);
        assert_pred!(result.is_none());
        let input = ["seeds: 79 14 55 13"];
        let result = parse_seeds(&input);
        assert_that!(result, some(eq(vec![79, 14, 55, 13])));
        Ok(())
    }

    #[googletest::test]
    fn test_map_src_to_dst() {
        let input = ["seed-to-soil map:", "50 98 2", "52 50 48"];
        let result = map_src_to_dst(&input).unwrap();
        assert_that!(result.get(&99), some(eq(&51u32)));
        assert_that!(result.get(&95), some(eq(&97u32)));
        assert_that!(result.get(&50), some(eq(&52u32)));
        assert_that!(result.get(&90), some(eq(&92u32)));
        let input = [];
        let result = map_src_to_dst(&input);
        assert_pred!(result.is_none());
    }

    #[test]
    fn test_get_dst_value() {
        let haystack = ["seed-to-soil map:", "50 98 2", "52 50 48"];
        let target = 99;
        let result = get_dst_value(&target, &haystack).unwrap();
        assert_that!(result, eq(51));

        let haystack = ["seed-to-soil map:", "23 76 20", "22 90 8"];
        let target = 97;
        let result = get_dst_value(&target, &haystack).unwrap();
        assert_that!(result, eq(29));

        let haystack = ["seed-to-soil map:", "9 88 38", "95 87 5"];
        let target = 1002;
        let result = get_dst_value(&target, &haystack).unwrap();
        assert_that!(result, eq(1002));

        let haystack = ["seed-to-soil map:", "9 88 38", "95 87 5"];
        let target = 7;
        let result = get_dst_value(&target, &haystack).unwrap();
        assert_that!(result, eq(7));

        let haystack = ["seed-to-soil map:"];
        let target = 7;
        let result = get_dst_value(&target, &haystack);
        assert_pred!(result.is_none());
    }
}
