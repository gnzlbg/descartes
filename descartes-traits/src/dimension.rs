//! Dimensions

use ops;
use typenum::Unsigned;

/// Ambient space dimension.
pub trait Ambient {
    type AD: Unsigned;
    fn ambient_dimension() -> usize {
        Self::AD::to_usize()
    }
    fn ambient_dimensions() -> ops::Range<usize> {
        0..Self::ambient_dimension()
    }
}

/// Geometry object dimension within its ambient space.
pub trait Object {
    type OD: Unsigned;
    fn object_dimension() -> usize {
        Self::OD::to_usize()
    }
    fn object_dimensions() -> ops::Range<usize> {
        0..Self::object_dimension()
    }
}
