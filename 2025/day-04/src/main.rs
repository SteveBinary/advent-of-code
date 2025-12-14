mod grid;

use grid::*;

fn main() {
    let input = include_str!("../input.txt");
    let mut grid = Grid::try_from(input).expect("input should be rectangular");

    let part_one_answer = paper_roll_locations_accessable_by_forklift(&grid).len();
    let part_two_answer =
        num_of_paper_roll_locations_accessable_by_forklift_when_removing_rolls(&mut grid);

    println!("answers:");
    println!(" - part one: {part_one_answer}");
    println!(" - part two: {part_two_answer}");
}

const PAPER_ROLL_ACCESSABLE_THRESHOLD: u32 = 3;

fn num_of_paper_roll_locations_accessable_by_forklift_when_removing_rolls(grid: &mut Grid) -> u32 {
    let accessable_paper_rolls_locations = paper_roll_locations_accessable_by_forklift(grid);
    let num_of_accessable_paper_rolls = accessable_paper_rolls_locations.len();

    if num_of_accessable_paper_rolls == 0 {
        0
    } else {
        for coord in accessable_paper_rolls_locations.iter() {
            grid.set(coord, Cell::Empty)
                .expect("coord should be in grid");
        }

        num_of_accessable_paper_rolls as u32
            + num_of_paper_roll_locations_accessable_by_forklift_when_removing_rolls(grid)
    }
}

fn paper_roll_locations_accessable_by_forklift(grid: &Grid) -> Vec<Coordinate> {
    let mut result = Vec::new();

    for (coord, cell) in grid.iter() {
        if *cell == Cell::PaperRoll
            && count_neighbors(grid, &coord) <= PAPER_ROLL_ACCESSABLE_THRESHOLD
        {
            result.push(coord);
        }
    }

    result
}

fn count_neighbors(grid: &Grid, coord: &Coordinate) -> u32 {
    let mut neighbors = 0;

    for x in coord.x.saturating_sub(1)..=coord.x.saturating_add(1) {
        for y in coord.y.saturating_sub(1)..=coord.y.saturating_add(1) {
            if x == coord.x && y == coord.y {
                continue;
            }

            if let Some(cell) = grid.at(&Coordinate { x, y })
                && *cell == Cell::PaperRoll
            {
                neighbors += 1;
            };
        }
    }

    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

    #[test]
    fn num_of_paper_rolls_accessable_by_forklift_works() {
        let grid = Grid::try_from(INPUT).expect("input should be rectangular");
        let result = paper_roll_locations_accessable_by_forklift(&grid);
        let expected = vec![
            Coordinate { x: 2, y: 0 },
            Coordinate { x: 3, y: 0 },
            Coordinate { x: 5, y: 0 },
            Coordinate { x: 6, y: 0 },
            Coordinate { x: 8, y: 0 },
            Coordinate { x: 0, y: 1 },
            Coordinate { x: 6, y: 2 },
            Coordinate { x: 0, y: 4 },
            Coordinate { x: 9, y: 4 },
            Coordinate { x: 0, y: 7 },
            Coordinate { x: 0, y: 9 },
            Coordinate { x: 2, y: 9 },
            Coordinate { x: 8, y: 9 },
        ];
        assert_eq!(expected, result);
    }

    #[test]
    fn num_of_paper_roll_locations_accessable_by_forklift_when_removing_rolls_works() {
        let mut grid = Grid::try_from(INPUT).expect("input should be rectangular");
        let result =
            num_of_paper_roll_locations_accessable_by_forklift_when_removing_rolls(&mut grid);
        let expected = 43;
        assert_eq!(expected, result);
    }
}
