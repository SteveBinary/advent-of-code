fn main() {
    let input = include_str!("../input.txt");
    let battery_banks = parse_input(input);

    let part_one_answer = max_joltage(&battery_banks, 2);
    let part_two_answer = max_joltage(&battery_banks, 12);

    println!("answers:");
    println!(" - part one: {part_one_answer}");
    println!(" - part two: {part_two_answer}");
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|char| char.to_digit(10))
                .map(|digit| digit as u64)
                .collect()
        })
        .collect()
}

fn max_joltage(banks: &[Vec<u64>], num_of_turned_on_batteries: usize) -> u64 {
    banks
        .iter()
        .map(|bank| find_max_jolts_of_battery_bank(bank, num_of_turned_on_batteries))
        .sum()
}

fn find_max_jolts_of_battery_bank(bank: &[u64], num_of_turned_on_batteries: usize) -> u64 {
    let mut start_index = 0;
    let mut max_jolts = 0;

    for n in (1..=num_of_turned_on_batteries).rev() {
        let (max_index, max_num) = find_max_with_index(&bank[start_index..=(bank.len() - n)]);
        start_index += max_index + 1;
        max_jolts += max_num * 10_u64.pow(n as u32 - 1)
    }

    max_jolts
}

fn find_max_with_index(nums: &[u64]) -> (usize, u64) {
    let mut max_index = 0;
    let mut max = nums[max_index];

    for (index, num) in nums[1..].iter().enumerate() {
        if *num > max {
            max = *num;
            max_index = index + 1;
        }
    }

    (max_index, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "987654321111111\n811111111111119\n234234234234278\n818181911112111";

    #[test]
    fn parse_input_should_work() {
        let battery_banks = parse_input(INPUT);
        let expected = vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ];
        assert_eq!(expected, battery_banks);
    }

    #[test]
    fn part_one_find_max_jolts_of_battery_bank_works() {
        let battery_banks = parse_input(INPUT);
        let expected = vec![98, 89, 78, 92];
        let actual: Vec<u64> = battery_banks
            .iter()
            .map(|bank| find_max_jolts_of_battery_bank(&bank, 2))
            .collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn part_two_find_max_jolts_of_battery_bank_works() {
        let battery_banks = parse_input(INPUT);
        let expected = vec![987654321111, 811111111119, 434234234278, 888911112111];
        let actual: Vec<u64> = battery_banks
            .iter()
            .map(|bank| find_max_jolts_of_battery_bank(&bank, 12))
            .collect();
        assert_eq!(expected, actual);
    }
}
