//! Vector

use descartes_traits::{dimension, Vector};
use generic_array::{ArrayLength, GenericArray};
use num_traits::{real::Real, NumAssign};
use typenum::U1;

#[derive(Copy, Clone, Default)]
pub struct V<E, A>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
{
    x: GenericArray<E, A>,
}

impl<E, A> dimension::Ambient for V<E, A>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
{
    type AD = A;
}

impl<E, A> dimension::Object for V<E, A>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
{
    type OD = U1;
}

impl<E, A> ::ops::Index<usize> for V<E, A>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
{
    type Output = E;
    fn index(&self, i: usize) -> &Self::Output {
        &self.x[i]
    }
}

impl<E, A> ::ops::IndexMut<usize> for V<E, A>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
{
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.x[i]
    }
}

impl<E, A> ::ops::Add for V<E, A>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
{
    type Output = Self;
    fn add(self, o: Self) -> Self::Output {
        let mut r = self;
        for i in 0..A::to_usize() {
            r[i] = self[i] + o[i];
        }
        r
    }
}

impl<E, A> ::ops::Sub for V<E, A>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
{
    type Output = Self;
    fn sub(self, o: Self) -> Self::Output {
        let mut r = self;
        for i in 0..A::to_usize() {
            r[i] = self[i] - o[i];
        }
        r
    }
}

impl<E, A> ::ops::AddAssign for V<E, A>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
{
    fn add_assign(&mut self, o: Self) {
        for i in 0..A::to_usize() {
            self[i] = self[i] + o[i];
        }
    }
}

impl<E, A> ::ops::SubAssign for V<E, A>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
{
    fn sub_assign(&mut self, o: Self) {
        for i in 0..A::to_usize() {
            self[i] = self[i] - o[i];
        }
    }
}

impl<E, A> ::ops::AddAssign<E> for V<E, A>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
{
    fn add_assign(&mut self, o: E) {
        for i in 0..A::to_usize() {
            self[i] = self[i] + o;
        }
    }
}

impl<E, A> ::ops::SubAssign<E> for V<E, A>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
{
    fn sub_assign(&mut self, o: E) {
        for i in 0..A::to_usize() {
            self[i] = self[i] - o;
        }
    }
}

impl<E, A> ::ops::MulAssign<E> for V<E, A>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
{
    fn mul_assign(&mut self, o: E) {
        for i in 0..A::to_usize() {
            self[i] = self[i] * o;
        }
    }
}

impl<E, A> ::ops::DivAssign<E> for V<E, A>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
{
    fn div_assign(&mut self, o: E) {
        for i in 0..A::to_usize() {
            self[i] = self[i] / o;
        }
    }
}

impl<E, A> ::ops::Add<E> for V<E, A>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
{
    type Output = Self;
    fn add(self, o: E) -> Self::Output {
        let mut r = self;
        for i in 0..A::to_usize() {
            r[i] = self[i] + o;
        }
        r
    }
}

impl<E, A> ::ops::Sub<E> for V<E, A>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
{
    type Output = Self;
    fn sub(self, o: E) -> Self::Output {
        let mut r = self;
        for i in 0..A::to_usize() {
            r[i] = self[i] - o;
        }
        r
    }
}

impl<E, A> ::ops::Mul<E> for V<E, A>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
{
    type Output = Self;
    fn mul(self, o: E) -> Self::Output {
        let mut r = self;
        for i in 0..A::to_usize() {
            r[i] = self[i] * o;
        }
        r
    }
}

impl<E, A> ::ops::Div<E> for V<E, A>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
{
    type Output = Self;
    fn div(self, o: E) -> Self::Output {
        let mut r = self;
        for i in 0..A::to_usize() {
            r[i] = self[i] / o;
        }
        r
    }
}

impl<E, A> ::ops::Neg for V<E, A>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut r = self;
        for i in 0..A::to_usize() {
            r[i] = -self[i];
        }
        r
    }
}

impl<E, A> Vector for V<E, A>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
{
    unsafe fn uninitialized_values() -> Self {
        ::mem::uninitialized()
    }
}

impl<E, A> ::fmt::Debug for V<E, A>
where
    E: Real + NumAssign + Default + ::fmt::Debug,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
{
    fn fmt(&self, w: &mut ::fmt::Formatter<'_>) -> Result<(), ::fmt::Error> {
        write!(w, "(")?;
        for i in 0..A::to_usize() {
            if i < A::to_usize() {
                write!(w, "{:?}, ", self[i])?;
            } else {
                write!(w, "{:?})", self[i])?;
            }
        }
        Ok(())
    }
}
