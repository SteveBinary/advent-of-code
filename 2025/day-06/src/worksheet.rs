use std::fmt::Debug;

pub struct Worksheet {
    pub number_lines_part_one: Vec<Vec<u64>>,
    pub number_columns_part_two: Vec<Vec<u64>>,
    pub operators: Vec<Operator>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Addition,
    Multiplication,
}

#[derive(Debug)]
pub struct WorkspaceCreationError;

impl TryFrom<&str> for Worksheet {
    type Error = WorkspaceCreationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines_iter = value.lines();

        let Some(operators_line) = lines_iter.next_back() else {
            return Err(WorkspaceCreationError);
        };

        let number_lines = lines_iter.collect::<Vec<&str>>();

        let line_length = operators_line.len();
        let mut operators = Vec::new();

        // the operator is at the left side of each number column
        // -> we use the operator to calculate where the columns are
        let mut column_start_indecies = Vec::new();

        // --- compute part one ----------------------------------------

        for (index, character) in operators_line.chars().enumerate() {
            match character {
                '+' => {
                    column_start_indecies.push(index);
                    operators.push(Operator::Addition);
                }
                '*' => {
                    column_start_indecies.push(index);
                    operators.push(Operator::Multiplication);
                }
                ' ' => {}
                _ => return Err(WorkspaceCreationError),
            };
        }

        let number_lines_part_one = number_lines
            .iter()
            .map(|line| {
                line.split_ascii_whitespace()
                    .filter_map(|element| element.parse::<u64>().ok())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // --- compute part two ----------------------------------------

        let mut number_columns_part_two = Vec::new();

        // We need add a 'fake start' of a column so that the windows created
        // can be interpreted as the start and end of a number column.
        column_start_indecies.push(line_length + 1);
        let column_ranges_inclusive = column_start_indecies
            .windows(2)
            .map(|arr| (arr[0], arr[1] - 2));

        for (column_start_index, column_end_index) in column_ranges_inclusive {
            let mut column_numbers = Vec::with_capacity(column_end_index - column_start_index + 1);

            for i in column_start_index..=column_end_index {
                let mut number_string = String::with_capacity(number_lines.len());

                for line in number_lines.iter() {
                    number_string.push(line.to_string().chars().nth(i).unwrap());
                }

                column_numbers.push(
                    number_string
                        .trim()
                        .parse::<u64>()
                        .expect("number should be"),
                );
            }

            number_columns_part_two.push(column_numbers);
        }

        Ok(Worksheet {
            number_lines_part_one,
            number_columns_part_two,
            operators,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn parse_input_works() {
        let worksheet = Worksheet::try_from(INPUT).expect("worksheet should be valid");

        let expected_number_lines_part_one = vec![
            vec![123, 328, 51, 64],
            vec![45, 64, 387, 23],
            vec![6, 98, 215, 314],
        ];
        let expected_number_lines_part_two = vec![
            vec![1, 24, 356],
            vec![369, 248, 8],
            vec![32, 581, 175],
            vec![623, 431, 4],
        ];
        let expected_operators = vec![
            Operator::Multiplication,
            Operator::Addition,
            Operator::Multiplication,
            Operator::Addition,
        ];

        assert_eq!(
            expected_number_lines_part_one,
            worksheet.number_lines_part_one
        );
        assert_eq!(
            expected_number_lines_part_two,
            worksheet.number_columns_part_two
        );
        assert_eq!(expected_operators, worksheet.operators);
    }
}
