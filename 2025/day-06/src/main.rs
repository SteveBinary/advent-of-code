mod worksheet;

use std::iter::zip;

use worksheet::*;

fn main() {
    let input = include_str!("../input.txt");
    let worksheet = Worksheet::try_from(input).expect("worksheet should be valid");

    let part_one_answer = solve_part_one(&worksheet);
    let part_two_answer = solve_part_two(&worksheet);

    println!("answers:");
    println!(" - part one: {part_one_answer}");
    println!(" - part two: {part_two_answer}");
}

fn solve_part_one(worksheet: &Worksheet) -> u64 {
    let mut result = 0;

    for (index, operator) in worksheet.operators.iter().enumerate() {
        let elements_to_operate_on = worksheet
            .number_lines_part_one
            .iter()
            .map(|elements| elements[index]);

        result += match operator {
            Operator::Addition => elements_to_operate_on.sum::<u64>(),
            Operator::Multiplication => elements_to_operate_on.product(),
        };
    }

    result
}

fn solve_part_two(worksheet: &Worksheet) -> u64 {
    zip(&worksheet.number_columns_part_two, &worksheet.operators)
        .map(|(numbers, operator)| match operator {
            Operator::Addition => numbers.iter().sum::<u64>(),
            Operator::Multiplication => numbers.iter().product(),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn solve_part_one_works() {
        let worksheet = Worksheet::try_from(INPUT).expect("worksheet should be valid");
        let answer = solve_part_one(&worksheet);
        let expected = 4277556;
        assert_eq!(expected, answer);
    }

    #[test]
    fn solve_part_two_works() {
        let worksheet = Worksheet::try_from(INPUT).expect("worksheet should be valid");
        let answer = solve_part_two(&worksheet);
        let expected = 3263827;
        assert_eq!(expected, answer);
    }
}
