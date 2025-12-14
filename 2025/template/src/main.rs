fn main() {
    let input = include_str!("../input.txt");
    let parsed = parse_input(input);

    let part_one_answer = 0;
    let part_two_answer = 0;

    println!("answers:");
    println!(" - part one: {part_one_answer}");
    println!(" - part two: {part_two_answer}");
}

fn parse_input(input: &str) -> () {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn parse_input_works() {
        let parsed = parse_input(INPUT);
        let expected = ();
        assert_eq!(expected, parsed);
    }
}
