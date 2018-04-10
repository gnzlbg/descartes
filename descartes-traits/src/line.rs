//! Line

use num_traits::{real::Real, NumAssign, Zero};
use ops;
use tolerance::Tolerance;

type E<T> = <T as ::associated::Number>::Number;

type V<T> = <T as ::associated::Vector>::Vector;
type P<T> = <T as ::associated::Point>::Point;

pub trait Line:
    Copy
    + Clone
    + ::dimension::Ambient
    + ::dimension::Object<OD = <Self as ::dimension::Ambient>::AD>
    + ::associated::Point
{
    unsafe fn uninitialized_values() -> Self;
    fn from_points(a: P<Self>, b: P<Self>) -> Self;
    fn origin(&self) -> P<Self>;
    fn direction(&self) -> V<Self>;
    fn point(&self, t: E<Self>) -> P<Self>;

    fn project(&self, p: P<Self>) -> P<Self> {
        use vector::Vector;
        use point::Point;
        use ops::Add;
        let a = self.origin();
        let ab = self.direction();
        a + ab * ((p - a).dot(ab) / ab.dot(ab))
    }

    fn parameter<T>(&self, p: P<Self>, tol: T) -> Option<E<Self>>
    where T: Tolerance<N = E<Self>>;
}
