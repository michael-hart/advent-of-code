use std::convert::TryFrom;
use crate::grid_construct_error::GridConstructError;

pub trait GridCell : TryFrom<char, Error = GridConstructError> + Sized + std::fmt::Debug {

}
