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

mod line;
pub use line::Line;

pub mod associated {
    use num_traits::{real::Real, NumAssign};
    use ops;

    pub trait Number {
        type Number: Real + NumAssign;
    }
    type E<T> = <T as Number>::Number;

    pub trait Vector: Number {
        type Vector: super::Vector<Number = E<Self>>;
    }
    type V<T> = <T as Vector>::Vector;


    pub trait Point: Vector {
        type Point: super::Point<Number = E<Self>, Vector = V<Self>>;
    }
}
