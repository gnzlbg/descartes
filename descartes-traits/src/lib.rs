extern crate num_traits;
extern crate typenum;

use std::ops;

mod tolerance;
pub use tolerance::Tolerance;

pub mod dimension;

mod vector;
pub use vector::Vector;

mod point;
pub use point::Point;

pub mod associated {
    use ::ops;
    use num_traits::{real::Real, NumAssign};

    pub trait Vector<E: Real + NumAssign> {
        type Vector: super::Vector + ops::Index<usize, Output = E>;
    }
}
