//! Vector

use num_traits::{NumAssign, One, Zero, real::Real};
use tolerance::Tolerance;
use typenum::{U1, U2, U3};
use ops;

type O<T> = <T as ops::Index<usize>>::Output;

pub trait Vector:
    Copy
    + Clone
    + ::dimension::Ambient
    + ::dimension::Object<OD = U1>
    + ops::Index<usize>
    + ops::IndexMut<usize>
    + ops::Add<Self, Output = Self>
    + ops::Sub<Self, Output = Self>
    + ops::AddAssign<Self>
    + ops::SubAssign
    + ops::AddAssign<O<Self>>
    + ops::SubAssign<O<Self>>
    + ops::MulAssign<O<Self>>
    + ops::DivAssign<O<Self>>
    + ops::Add<O<Self>, Output = Self>
    + ops::Sub<O<Self>, Output = Self>
    + ops::Mul<O<Self>, Output = Self>
    + ops::Div<O<Self>, Output = Self>
    + ops::Neg<Output = Self>
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

    fn base(i: usize) -> Self {
        let mut v = Self::null();
        v[i] = <O<Self> as One>::one();
        v
    }

    fn dot(&self, other: Self) -> O<Self> {
        let mut v = <O<Self> as Zero>::zero();
        for d in Self::ambient_dimensions() {
            v += self[d] * other[d];
        }
        v
    }
    fn norm2(&self) -> O<Self> {
        let mut v = <O<Self> as Zero>::zero();
        for d in Self::ambient_dimensions() {
            v += self[d].powi(2);
        }
        v
    }
    fn norm(&self) -> O<Self> {
        self.norm2().sqrt()
    }
    fn cross(&self, other: Self) -> Self
    where
        Self: ::dimension::Ambient<AD = U3>,
    {
        let mut v = Self::null();
        v[0] = self[1] * other[2] - self[2] * other[1];
        v[1] = self[2] * other[0] - self[0] * other[2];
        v[2] = self[0] * other[1] - self[1] * other[0];
        v
    }
    fn perp_product(&self, other: Self) -> O<Self> {
        match Self::ambient_dimension() {
            2 => self[0] * other[1] - self[1] * other[0],
            3 => {
                let mut v = Self::null();
                v[0] = self[1] * other[2] - self[2] * other[1];
                v[1] = self[2] * other[0] - self[0] * other[2];
                v[2] = self[0] * other[1] - self[1] * other[0];
                v.norm()
            }
            _ => unreachable!(),
        }
    }
    fn length(&self) -> O<Self> {
        self.norm()
    }

    fn parallel<T>(&self, other: Self) -> bool
    where
        T: Tolerance<N = O<Self>>,
    {
        match Self::ambient_dimension() {
            1 => true,
            2 => T::approx_eq(
                self.perp_product(other).abs(),
                <O<Self> as Zero>::zero(),
            ),
            3 => T::approx_eq(
                self.perp_product(other),
                <O<Self> as Zero>::zero(),
            ),
            _ => unreachable!(),
        }
    }
    fn rotate_90_ccw(&self) -> Self
    where
        Self: ::dimension::Ambient<AD = U2>,
    {
        let mut v = Self::null();
        v[0] = -self[1];
        v[1] = self[0];
        v
    }

    fn rotate_90_cw(&self) -> Self
    where
        Self: ::dimension::Ambient<AD = U2>,
    {
        let mut v = Self::null();
        v[0] = self[1];
        v[1] = -self[0];
        v
    }

    fn normal(&self) -> Self
    where
        Self: ::dimension::Ambient<AD = U2>,
    {
        self.rotate_90_ccw()
    }

    fn invert(&self) -> Self {
        let mut v = Self::null();
        for i in Self::ambient_dimensions() {
            v[i] = self[i] * -<O<Self> as One>::one();
        }
        v
    }
}
