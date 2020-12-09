use std::convert::TryFrom;

use crate::grid_cell::GridCell;
use crate::grid_construct_error::GridConstructError;
use crate::point::Point;

#[derive(Debug, PartialEq, Eq)]
pub struct Grid<Cell>
where
    Cell: GridCell + Sized
{
    grid: Vec<Vec<Cell>>,
}

impl<T> TryFrom<&str> for Grid<T>
where
    T: GridCell + Sized
{
    type Error = GridConstructError;
    fn try_from(raw: &str) -> Result<Self, Self::Error> {

        let grid_result = raw
            .lines()
            .map(Self::try_row_from)
            .collect();

        // Return early if previous function returned err
        let grid: Vec<Vec<T>> = match grid_result {
            Ok(g) => g,
            Err(e) => return Err(e),
        };

        // Check grid is not empty
        if grid.len() == 0 {
            return Err(GridConstructError::EmptyGrid);
        }

        // Check shape of grid
        let row_size = grid[0].len();
        for row in grid.iter().skip(1) {
            if row.len() != row_size {
                return Err(GridConstructError::MisshapenGrid);
            }
        }

        Ok(Grid { grid })
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: GridCell + std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for cell in row {
                std::fmt::Display::fmt(&cell, f)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl<T> Grid<T>
where
    T: GridCell + Sized
{
    fn try_row_from(raw: &str) -> Result<Vec<T>, GridConstructError> {
        raw
            .chars()
            .map(T::try_from)
            .collect()
    }

    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn cell_at(&self, point: &Point) -> Option<&T> {
        if let Some(row) = self.grid.get(point.y) {
            if let Some(cell) = row.get(point.x) {
                return Some(cell);
            }
        }

        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum TestCell { A, B }
    impl TryFrom<char> for TestCell {
        type Error = GridConstructError;
        fn try_from(c: char) -> Result<Self, Self::Error> {
            match c {
                'A' => Ok(TestCell::A),
                'B' => Ok(TestCell::B),
                _ => Err(GridConstructError::CellInvalid(c)),
            }
        }
    }
    impl std::fmt::Display for TestCell {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                TestCell::A => write!(f, "A"),
                TestCell::B => write!(f, "B"),
            }
        }
    }

    impl GridCell for TestCell {}

    #[test]
    fn test_row_from_works() {
        let row = "ABBA";
        let result = Grid::try_row_from(row);
        assert_eq!(result, Ok(vec![TestCell::A, TestCell::B, TestCell::B, TestCell::A]));
    }

    #[test]
    fn test_invalid_row_from_works() {
        let row = "CBA";
        let result : Result<Vec<TestCell>, GridConstructError> = Grid::try_row_from(row);
        assert_eq!(result, Err(GridConstructError::CellInvalid('C')));
    }

    #[test]
    fn create_simple_grid_works() {
        let grid = Grid::try_from("AAB\nABB");
        assert_eq!(grid, Ok(Grid { grid: vec![vec![TestCell::A, TestCell::A, TestCell::B], vec![TestCell::A, TestCell::B, TestCell::B]]}));
    }

    #[test]
    fn create_invalid_cell_grid_fails() {
        let grid : Result<Grid<TestCell>, GridConstructError> = Grid::try_from("AAB\nACB");
        assert_eq!(grid, Err(GridConstructError::CellInvalid('C')));
    }

    #[test]
    fn test_misshapen_grid_fails() {
        let grid : Result<Grid<TestCell>, GridConstructError> = Grid::try_from("AB\nA");
        assert_eq!(grid, Err(GridConstructError::MisshapenGrid));
    }

    #[test]
    fn test_carriage_returns_ignored() {
        let grid = Grid::try_from("A\r\nA");
        assert_eq!(grid, Ok(Grid { grid: vec![vec![TestCell::A], vec![TestCell::A]]}));
    }

    #[test]
    fn test_trailing_newline_ignored() {
        let grid = Grid::try_from("A\nA\n");
        assert_eq!(grid, Ok(Grid { grid: vec![vec![TestCell::A], vec![TestCell::A]]}));
    }

    #[test]
    fn test_empty_grid_fails() {
        let grid : Result<Grid<TestCell>, GridConstructError> = Grid::try_from("");
        assert_eq!(grid, Err(GridConstructError::EmptyGrid));
    }

    #[test]
    fn test_grid_dims_correct() {
        let grid: Grid<TestCell> = Grid::try_from("AAB\nABB").unwrap();
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 2);
    }

    #[test]
    fn test_cell_at_correct() {
        let grid: Grid<TestCell> = Grid::try_from("AAB\nABB").unwrap();
        assert_eq!(*grid.cell_at(&Point::new(0, 1)).unwrap(), TestCell::A);
        assert_eq!(*grid.cell_at(&Point::new(2, 1)).unwrap(), TestCell::B);
    }

    #[test]
    fn test_only_cell_is_correct() {
        let grid: Grid<TestCell> = Grid::try_from("AAAA\nAAAA\nABAA\nAAAA\n").unwrap();
        assert_eq!(*grid.cell_at(&Point::new(1, 2)).unwrap(), TestCell::B);
    }

    #[test]
    fn test_cell_at_out_of_bounds_gives_none() {
        let grid: Grid<TestCell> = Grid::try_from("AAB\nABB").unwrap();
        assert_eq!(grid.cell_at(&Point::new(2, 2)), None);
    }
}
