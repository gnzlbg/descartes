//! Point

use num_traits::{real::Real, NumAssign, Zero};
use ops;

type E<T> = <T as ::associated::Number>::Number;
type V<T> = <T as ::associated::Vector>::Vector;

pub trait Point:
    Copy
    + Clone
    + ::dimension::Ambient
    + ::dimension::Object<OD = <Self as ::dimension::Ambient>::AD>
    + ::associated::Vector
    + ops::Index<usize, Output = E<Self>>
    + ops::IndexMut<usize>
    + ops::Add<V<Self>>
    + ops::Sub<V<Self>>
    + ops::Sub<Self, Output = V<Self>>
{
    unsafe fn uninitialized_values() -> Self;

    fn null() -> Self {
        Self::constant(<E<Self> as Zero>::zero())
    }

    fn from_slice(s: &[E<Self>]) -> Self {
        let mut v = Self::null();
        assert!(s.len() == Self::ambient_dimension());
        for i in Self::ambient_dimensions() {
            v[i] = s[i];
        }
        v
    }
    fn constant(c: E<Self>) -> Self {
        let mut v = unsafe { Self::uninitialized_values() };
        for i in Self::ambient_dimensions() {
            v[i] = c;
        }
        v
    }

    fn centroid(&self) -> Self {
        *self
    }
}
