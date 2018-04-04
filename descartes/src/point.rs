//! Point

use descartes_traits::dimension;
use generic_array::{ArrayLength, GenericArray};
use num_traits::{NumAssign, real::Real};

#[derive(Copy, Clone, Default)]
pub struct P<E, A, D>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
    D: Copy + Default,
{
    x: GenericArray<E, A>,
    pub data: D,
}

impl<E, A, D> dimension::Ambient for P<E, A, D>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
    D: Copy + Default,
{
    type AD = A;
}

impl<E, A, D> dimension::Object for P<E, A, D>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
    D: Copy + Default,
{
    type OD = <Self as dimension::Ambient>::AD;
}

impl<E, A, D> ::ops::Index<usize> for P<E, A, D>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
    D: Copy + Default,
{
    type Output = E;
    fn index(&self, i: usize) -> &Self::Output {
        &self.x[i]
    }
}

impl<E, A, D> ::ops::IndexMut<usize> for P<E, A, D>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
    D: Copy + Default,
{
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.x[i]
    }
}

impl<E, A, D> ::descartes_traits::associated::Vector<E> for P<E, A, D>
    where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
GenericArray<E, A>: Copy + Default,
    D: Copy + Default,
{
    type Vector = super::V<E, A>;
}

impl<E, A, D> ::ops::Add<super::V<E,A>> for P<E, A, D>
    where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
GenericArray<E, A>: Copy + Default,
    D: Copy + Default,
{
    type Output = Self;
    fn add(mut self, o: super::V<E,A>) -> Self::Output {
        use descartes_traits::dimension::Ambient;
        for i in Self::ambient_dimensions() {
            self[i] += o[i];
        }
        self
    }
}

impl<E, A, D> ::descartes_traits::Point for P<E, A, D>
where
    E: Real + NumAssign + Default,
    A: ArrayLength<E> + Copy + Default,
    GenericArray<E, A>: Copy + Default,
    D: Copy + Default,
{
    unsafe fn uninitialized_values() -> Self {
        Self {
            x: ::mem::uninitialized(),
            data: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use ::{P, V, EmptyData};
    use typenum::{U1, U2, U3};
    type P1D = P<f32, U1, EmptyData>;
    type V1D = V<f32, U1>;
    type P2D = P<f32, U2, EmptyData>;
    type V2D = V<f32, U2>;
    type P3D = P<f32, U3, EmptyData>;
    type V3D = V<f32, U3>;

    #[test]
    fn point() {
        use descartes_traits::{Vector, Point};
        {  // 1D:
            let x = P1D::constant(1.);
            let y = V1D::constant(2.);
            let z = x + y;
            for i in 0..1 {
                assert_eq!(z[i], 3.);
            }
        }

        {  // 2D:
            let x = P2D::constant(1.);
            let y = V2D::constant(2.);
            let z = x + y;
            for i in 0..2 {
                assert_eq!(z[i], 3.);
            }
        }

        {  // 3D:
            let x = P3D::constant(1.);
            let y = V3D::constant(2.);
            let z = x + y;

            for i in 0..3 {
                assert_eq!(z[i], 3.);
            }
        }
    }
}
