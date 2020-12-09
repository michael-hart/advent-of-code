use std::convert::TryFrom;
use grid::{GridCell, GridConstructError};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MapCell {
    Free,
    Tree,
}

impl std::fmt::Display for MapCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapCell::Free => write!(f, "."),
            MapCell::Tree => write!(f, "#"),
        }
    }
}

impl TryFrom<char> for MapCell {
    type Error = GridConstructError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(MapCell::Free),
            '#' => Ok(MapCell::Tree),
            _ => Err(GridConstructError::CellInvalid(c)),
        }
    }
}
impl GridCell for MapCell {}
