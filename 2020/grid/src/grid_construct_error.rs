#[derive(Debug, PartialEq, Eq)]
pub enum GridConstructError {
    CellInvalid(char),
    MisshapenGrid,
}
