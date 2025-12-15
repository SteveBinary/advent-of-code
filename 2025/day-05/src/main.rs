mod inventory;

use inventory::*;

fn main() {
    let input = include_str!("../input.txt");
    let inventory = Inventory::try_from(input).expect("inventory should be valid");

    let part_one_answer = num_of_fresh_and_available_ingredients(&inventory);
    let part_two_answer = num_of_possible_fresh_ingredients(&inventory);

    println!("answers:");
    println!(" - part one: {part_one_answer}");
    println!(" - part two: {part_two_answer}");
}

fn num_of_fresh_and_available_ingredients(inventory: &Inventory) -> usize {
    inventory
        .available_ingredient_ids
        .iter()
        .filter(|id| {
            inventory
                .fresh_ingredient_id_ranges
                .iter()
                .any(|range| range.contains(id))
        })
        .count()
}

fn num_of_possible_fresh_ingredients(inventory: &Inventory) -> u64 {
    // Creating a hashset over all ids allowed by the ranges would be too costly.
    // It would require too much memory.
    // Sorting the ranges is OK, because there a much much fewer ranges then possible values in those ranges.

    let mut ranges_sorted_by_start = inventory.fresh_ingredient_id_ranges.clone();
    ranges_sorted_by_start.sort_by_key(|r| *r.start());

    let mut num = 0;
    let mut latest_highest_id = 0;

    for range in ranges_sorted_by_start.iter() {
        let start = *range.start();
        let end = *range.end();

        if start > latest_highest_id {
            num += end - start + 1;
            latest_highest_id = end;
        } else if end > latest_highest_id {
            num += end - latest_highest_id;
            latest_highest_id = end;
        }
    }

    num
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";

    #[test]
    fn num_of_fresh_and_available_ingredients_works() {
        let inventory = Inventory::try_from(INPUT).expect("inventory should be valid");
        let result = num_of_fresh_and_available_ingredients(&inventory);
        let expected = 3;
        assert_eq!(expected, result);
    }

    #[test]
    fn num_of_possible_fresh_ingredients_works() {
        let inventory = Inventory::try_from(INPUT).expect("inventory should be valid");
        let result = num_of_possible_fresh_ingredients(&inventory);
        let expected = 14;
        assert_eq!(expected, result);
    }
}
