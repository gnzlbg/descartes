//! Vector

use num_traits::{real::Real, NumAssign, One, Zero};
use ops;
use tolerance::Tolerance;
use typenum::{U1, U2, U3};

type E<T> = <T as ::associated::Number>::Number;

pub trait Vector:
    Copy
    + Clone
    + ::dimension::Ambient
    + ::dimension::Object<OD = U1>
    + ::associated::Number
    + ops::Index<usize, Output = E<Self>>
    + ops::IndexMut<usize>
    + ops::Add<Self, Output = Self>
    + ops::Sub<Self, Output = Self>
    + ops::AddAssign<Self>
    + ops::SubAssign
    + ops::AddAssign<E<Self>>
    + ops::SubAssign<E<Self>>
    + ops::MulAssign<E<Self>>
    + ops::DivAssign<E<Self>>
    + ops::Add<E<Self>, Output = Self>
    + ops::Sub<E<Self>, Output = Self>
    + ops::Mul<E<Self>, Output = Self>
    + ops::Div<E<Self>, Output = Self>
    + ops::Neg<Output = Self>
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

    fn base(i: usize) -> Self {
        let mut v = Self::null();
        v[i] = <E<Self> as One>::one();
        v
    }

    fn dot(&self, other: Self) -> E<Self> {
        let mut v = <E<Self> as Zero>::zero();
        for d in Self::ambient_dimensions() {
            v += self[d] * other[d];
        }
        v
    }
    fn norm2(&self) -> E<Self> {
        let mut v = <E<Self> as Zero>::zero();
        for d in Self::ambient_dimensions() {
            v += self[d].powi(2);
        }
        v
    }
    fn norm(&self) -> E<Self> {
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
    fn perp_product(&self, other: Self) -> E<Self> {
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
    fn length(&self) -> E<Self> {
        self.norm()
    }

    fn parallel<T>(&self, other: Self) -> bool
    where
        T: Tolerance<N = E<Self>>,
    {
        match Self::ambient_dimension() {
            1 => true,
            2 => T::approx_eq(
                self.perp_product(other).abs(),
                <E<Self> as Zero>::zero(),
            ),
            3 => T::approx_eq(
                self.perp_product(other),
                <E<Self> as Zero>::zero(),
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
            v[i] = self[i] * -<E<Self> as One>::one();
        }
        v
    }
}
