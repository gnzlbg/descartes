extern crate descartes_traits;
extern crate generic_array;
extern crate num_traits;
extern crate typenum;

use std::{fmt, marker, mem, ops};

pub mod tolerance;

mod vector;
pub use vector::V;

mod point;
pub use point::P;

mod line;
pub use line::L;

#[derive(Copy, Clone, Default)]
pub struct EmptyData();
