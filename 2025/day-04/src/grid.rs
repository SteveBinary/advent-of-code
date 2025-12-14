pub struct Grid {
    cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    PaperRoll,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct CoordinateNotInGrid;

impl Grid {
    pub fn iter<'a>(&'a self) -> GridIterator<'a> {
        GridIterator {
            index: 0,
            grid: self,
        }
    }

    pub fn at(&self, coord: &Coordinate) -> Option<&Cell> {
        if coord.x >= self.width || coord.y >= self.height {
            return None;
        }

        let cell_index = coord.x + self.width * coord.y;

        if cell_index >= self.cells.len() {
            return None;
        }

        self.cells.get(cell_index)
    }

    pub fn set(&mut self, coord: &Coordinate, cell: Cell) -> Result<(), CoordinateNotInGrid> {
        if coord.x >= self.width || coord.y >= self.height {
            return Err(CoordinateNotInGrid);
        }

        let cell_index = coord.x + self.width * coord.y;

        if cell_index >= self.cells.len() {
            return Err(CoordinateNotInGrid);
        }

        self.cells[cell_index] = cell;

        Ok(())
    }
}

#[derive(Debug)]
pub enum GridConstructionError {
    ValueIsNotOfRectangularShape,
    DimensionsCannotBeZero,
}

impl TryFrom<&str> for Grid {
    type Error = GridConstructionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lines = value.lines().collect::<Vec<_>>();
        let height = lines.len();

        let cells: Vec<Cell> = lines
            .iter()
            .flat_map(|line| {
                line.chars().map(|char| {
                    if char == '@' {
                        Cell::PaperRoll
                    } else {
                        Cell::Empty
                    }
                })
            })
            .collect();

        if height == 0 {
            return Err(GridConstructionError::DimensionsCannotBeZero);
        }

        let width = cells.len() / height;

        if width == 0 {
            return Err(GridConstructionError::DimensionsCannotBeZero);
        }

        if width * height != cells.len() {
            return Err(GridConstructionError::ValueIsNotOfRectangularShape);
        }

        Ok(Self {
            cells,
            width,
            height,
        })
    }
}

pub struct GridIterator<'a> {
    index: usize,
    grid: &'a Grid,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = (Coordinate, &'a Cell);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.grid.cells.len() {
            let x = self.index % self.grid.width;
            let y = (self.index - x) / self.grid.width;
            let cell = &self.grid.cells[self.index];
            self.index += 1;
            Some((Coordinate { x, y }, cell))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

    #[test]
    fn parse_input_works() {
        use Cell::*;

        let grid = Grid::try_from(INPUT).expect("input should be rectangular");
        let expected_cells = vec![
            Empty, Empty, PaperRoll, PaperRoll, Empty, PaperRoll, PaperRoll, PaperRoll, PaperRoll,
            Empty, PaperRoll, PaperRoll, PaperRoll, Empty, PaperRoll, Empty, PaperRoll, Empty,
            PaperRoll, PaperRoll, PaperRoll, PaperRoll, PaperRoll, PaperRoll, PaperRoll, Empty,
            PaperRoll, Empty, PaperRoll, PaperRoll, PaperRoll, Empty, PaperRoll, PaperRoll,
            PaperRoll, PaperRoll, Empty, Empty, PaperRoll, Empty, PaperRoll, PaperRoll, Empty,
            PaperRoll, PaperRoll, PaperRoll, PaperRoll, Empty, PaperRoll, PaperRoll, Empty,
            PaperRoll, PaperRoll, PaperRoll, PaperRoll, PaperRoll, PaperRoll, PaperRoll, Empty,
            PaperRoll, Empty, PaperRoll, Empty, PaperRoll, Empty, PaperRoll, Empty, PaperRoll,
            PaperRoll, PaperRoll, PaperRoll, Empty, PaperRoll, PaperRoll, PaperRoll, Empty,
            PaperRoll, PaperRoll, PaperRoll, PaperRoll, Empty, PaperRoll, PaperRoll, PaperRoll,
            PaperRoll, PaperRoll, PaperRoll, PaperRoll, PaperRoll, Empty, PaperRoll, Empty,
            PaperRoll, Empty, PaperRoll, PaperRoll, PaperRoll, Empty, PaperRoll, Empty,
        ];
        assert_eq!(expected_cells, grid.cells);
    }
}
