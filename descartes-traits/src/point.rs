//! Point

use num_traits::{NumAssign, Zero, real::Real};
use ops;

type O<T> = <T as ops::Index<usize>>::Output;

#[allow(dead_code)] // FIXME: is this a rustc bug?
type V<T> = <T as ::associated::Vector<O<T>>>::Vector;

pub trait Point:
    Copy
    + Clone
    + ::dimension::Ambient
    + ::dimension::Object<OD = <Self as ::dimension::Ambient>::AD>
    + ops::Index<usize>
    + ops::IndexMut<usize>
    + ::associated::Vector<O<Self>>
    + ops::Add<V<Self>>
    + ops::Sub<V<Self>>
    + ops::Sub<Self, Output = V<Self>>
where
    Self: Sized,
    <Self as ops::Index<usize>>::Output: Real + NumAssign,
{
    unsafe fn uninitialized_values() -> Self;

    fn null() -> Self {
        Self::constant(<O<Self> as Zero>::zero())
    }

    fn from_slice(s: &[O<Self>]) -> Self {
        let mut v = Self::null();
        assert!(s.len() == Self::ambient_dimension());
        for i in Self::ambient_dimensions() {
            v[i] = s[i];
        }
        v
    }
    fn constant(c: O<Self>) -> Self {
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
