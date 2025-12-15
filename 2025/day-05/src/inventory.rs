use std::ops::RangeInclusive;

pub type IngredientId = u64;

pub struct Inventory {
    pub fresh_ingredient_id_ranges: Vec<RangeInclusive<IngredientId>>,
    pub available_ingredient_ids: Vec<IngredientId>,
}

#[derive(Debug)]
pub enum InventoryCreationError {
    InvalidIngredientIdRange,
    InvalidIngredientId,
}

impl TryFrom<&str> for Inventory {
    type Error = InventoryCreationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut id_ranges = Vec::new();
        let mut available_ids = Vec::new();

        let mut parsing_ranges = true;
        for line in value.lines() {
            if parsing_ranges && line.is_empty() {
                parsing_ranges = false;
                continue;
            }

            if parsing_ranges {
                if let Some((range_start, range_end)) = line.trim().split_once("-") {
                    match (
                        range_start.parse::<IngredientId>(),
                        range_end.parse::<IngredientId>(),
                    ) {
                        (Ok(range_start), Ok(range_end)) => id_ranges.push(range_start..=range_end),
                        _ => return Err(InventoryCreationError::InvalidIngredientIdRange),
                    }
                } else {
                    return Err(InventoryCreationError::InvalidIngredientIdRange);
                }
            } else {
                match line.trim().parse::<IngredientId>() {
                    Err(_) => return Err(InventoryCreationError::InvalidIngredientId),
                    Ok(id) => available_ids.push(id),
                }
            }
        }

        Ok(Inventory {
            fresh_ingredient_id_ranges: id_ranges,
            available_ingredient_ids: available_ids,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";

    #[test]
    fn inventory_parsing_works() {
        let inventory = Inventory::try_from(INPUT).expect("inventory should be valid");

        let expected_ranges = vec![3..=5, 10..=14, 16..=20, 12..=18];

        let expected_available_ids = vec![1, 5, 8, 11, 17, 32];

        assert_eq!(expected_ranges, inventory.fresh_ingredient_id_ranges);
        assert_eq!(expected_available_ids, inventory.available_ingredient_ids);
    }
}
