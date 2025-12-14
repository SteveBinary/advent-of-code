use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../input.txt");
    let ranges = parse_input(input);

    let part_one_invalid_ids = find_invalid_ids(&ranges, part_one_is_invalid_id);
    let part_two_invalid_ids = find_invalid_ids(&ranges, part_two_is_invalid_id);

    let part_one_answer = part_one_invalid_ids.iter().sum::<Id>();
    let part_two_answer = part_two_invalid_ids.iter().sum::<Id>();

    println!("answers:");
    println!(" - part one: {part_one_answer}");
    println!(" - part two: {part_two_answer}");
}

type Id = u64;
type IdRange = RangeInclusive<Id>;

fn parse_input(input: &str) -> Vec<IdRange> {
    input
        .split(',')
        .map(|range| range.split('-'))
        .map(|range| {
            range.map(|range_value| {
                range_value
                    .trim()
                    .parse::<Id>()
                    .expect("IDs are always a positive integer")
            })
        })
        .map(|mut range| {
            IdRange::new(
                range.next().expect("a range has a start and an end"),
                range.next().expect("a range has a start and an end"),
            )
        })
        .collect()
}

fn find_invalid_ids(id_ranges: &[IdRange], is_invalid: fn(&str) -> bool) -> Vec<Id> {
    id_ranges
        .iter()
        .flat_map(|range| range.clone().map(|value| (value, value.to_string())))
        .filter_map(|r| if is_invalid(&r.1) { Some(r.0) } else { None })
        .collect()
}

fn part_one_is_invalid_id(id: &str) -> bool {
    let (left, right) = id.split_at(id.len() / 2);
    return left == right;
}

fn part_two_is_invalid_id(id: &str) -> bool {
    let max_substr_len = id.len() / 2;

    'outer: for substr_len in (1..=max_substr_len).filter(|l| id.len() % l == 0) {
        let (substr_to_check, rest) = id.split_at(substr_len);

        for window_into_rest in rest.chars().collect::<Vec<_>>().chunks(substr_len) {
            for (char_of_substr_to_check, char_of_rest) in
                substr_to_check.chars().zip(window_into_rest)
            {
                if char_of_substr_to_check != *char_of_rest {
                    continue 'outer;
                }
            }
        }

        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn parse_should_work() {
        let ranges = parse_input(INPUT);
        let expected = vec![
            11..=22,
            95..=115,
            998..=1012,
            1188511880..=1188511890,
            222220..=222224,
            1698522..=1698528,
            446443..=446449,
            38593856..=38593862,
            565653..=565659,
            824824821..=824824827,
            2121212118..=2121212124,
        ];
        assert_eq!(expected, ranges);
    }

    #[test]
    fn part_one_find_invalid_ids_works() {
        let ranges = parse_input(INPUT);
        let invalid_ids = find_invalid_ids(&ranges, part_one_is_invalid_id);
        let expected = vec![11, 22, 99, 1010, 1188511885, 222222, 446446, 38593859];
        assert_eq!(expected, invalid_ids);
    }

    #[test]
    fn part_two_find_invalid_ids_works() {
        let ranges = parse_input(INPUT);
        let invalid_ids = find_invalid_ids(&ranges, part_two_is_invalid_id);
        let expected = vec![
            11, 22, 99, 111, 999, 1010, 1188511885, 222222, 446446, 38593859, 565656, 824824824,
            2121212121,
        ];
        assert_eq!(expected, invalid_ids);
    }
}
