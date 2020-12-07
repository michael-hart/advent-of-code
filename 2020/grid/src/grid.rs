use std::convert::TryFrom;

use crate::grid_cell::GridCell;
use crate::grid_construct_error::GridConstructError;

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
        let grid = match grid_result {
            Ok(g) => g,
            Err(e) => return Err(e),
        };

        // Check shape of grid
        // let row_size = grid[0].len()
        // Check all other rows against that

        //TODO: ignore carriage returns
        // TODO: ignore blank lines
        // TODO: check empty grid

        Ok(Grid { grid })
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
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
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

}
