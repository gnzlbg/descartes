//! Implementations of the `descartes::Tolerance` trait.

use descartes_traits::Tolerance;

/// Default tolerance for primitive types.
pub struct Default<T>(::marker::PhantomData<T>);

impl Tolerance for Default<f32> {
    type N = f32;
    fn absolute() -> Self::N {
        1e-6
    }
    fn relative() -> Self::N {
        1e-8
    }
}

impl Tolerance for Default<f64> {
    type N = f64;
    fn absolute() -> Self::N {
        1e-15
    }
    fn relative() -> Self::N {
        1e-13
    }
}
