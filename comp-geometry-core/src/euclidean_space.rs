use std::ops::Sub;

use crate::{Scalar, inner_space::InnerSpace};

pub trait EuclideanSpace:
    Copy + Sub<Self, Output = <Self as EuclideanSpace>::Diff>
{
    type Scalar: Scalar;
    type Diff: InnerSpace<Scalar = Self::Scalar>;
}
