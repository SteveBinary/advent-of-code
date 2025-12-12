fn main() {
    let input = include_str!("../input.txt");

    let (part_one_answer, part_two_answer) = find_password(input);

    println!("answers:");
    println!(" - part one: {part_one_answer}");
    println!(" - part two: {part_two_answer}");
}

const NUMBER_OF_DIAL_POSITIONS: i64 = 100;

fn find_password(input: &str) -> (i64, i64) {
    let mut part_one_answer = 0;
    let mut part_two_answer = 0;

    let mut dial_position = 50;

    for line in input.lines() {
        let (rotation_direction, rotation_amount) = line.split_at(1);

        let rotation_multiplier = match rotation_direction {
            "L" => -1,
            "R" => 1,
            _ => unreachable!(),
        };

        let rotation_amount = rotation_amount
            .parse::<i64>()
            .expect("amount should always be an integer");

        if rotation_multiplier > 0 {
            part_two_answer += (dial_position + rotation_amount) / NUMBER_OF_DIAL_POSITIONS;
        } else if dial_position <= rotation_amount {
            let additional = if dial_position == 0 { 0 } else { 1 };
            part_two_answer +=
                (-dial_position + rotation_amount) / NUMBER_OF_DIAL_POSITIONS + additional;
        }

        dial_position += rotation_multiplier * rotation_amount;
        dial_position = dial_position.rem_euclid(NUMBER_OF_DIAL_POSITIONS);

        assert!(dial_position >= 0);

        if dial_position == 0 {
            part_one_answer += 1;
        }
    }

    (part_one_answer, part_two_answer)
}
