//! Tolerance

use num_traits::{real::Real, NumAssign};

/// Tolerance.
pub trait Tolerance {
    type N: Real + NumAssign;
    fn absolute() -> Self::N;
    fn relative() -> Self::N;
    fn approx_eq(a: Self::N, b: Self::N) -> bool {
        (a - b).abs()
            <= Self::absolute().max(Self::relative() * a.abs().max(b.abs()))
    }
}
