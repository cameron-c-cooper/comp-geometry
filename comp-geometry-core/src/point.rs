use crate::vector::{Vector2, Vector3, Vector4};

macro_rules! impl_point {
    ($point:ident, $vec:ident { $($field:ident),+ $(,)? }) => {
        #[repr(C)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $point<T> {
            $(pub $field: T),+
        }

        impl<T> $point<T> {
            #[inline]
            pub const fn new($($field: T),+) -> Self {
                Self { $($field),+ }
            }
        }

        // Point - Point = Vector
        impl<T: $crate::Scalar> std::ops::Sub<$point<T>> for $point<T> {
            type Output = $vec<T>;
            #[inline]
            fn sub(self, rhs: $point<T>) -> $vec<T> {
                $vec::new($(self.$field - rhs.$field),+)
            }
        }

        // Point + Vector = Point
        impl<T: $crate::Scalar> std::ops::Add<$vec<T>> for $point<T> {
            type Output = $point<T>;
            #[inline]
            fn add(self, rhs: $vec<T>) -> $point<T> {
                $point::new($(self.$field + rhs.$field),+)
            }
        }

        // Point - Vector = Point
        impl<T: $crate::Scalar> std::ops::Sub<$vec<T>> for $point<T> {
            type Output = $point<T>;
            #[inline]
            fn sub(self, rhs: $vec<T>) -> $point<T> {
                $point::new($(self.$field - rhs.$field),+)
            }
        }

        impl<T: $crate::Scalar> std::ops::AddAssign<$vec<T>> for $point<T> {
            #[inline]
            fn add_assign(&mut self, rhs: $vec<T>) {
                $(self.$field += rhs.$field;)+
            }
        }

        impl<T: $crate::Scalar> std::ops::SubAssign<$vec<T>> for $point<T> {
            #[inline]
            fn sub_assign(&mut self, rhs: $vec<T>) {
                $(self.$field -= rhs.$field;)+
            }
        }

        impl<T: $crate::SignedScalar> $crate::euclidean_space::EuclideanSpace for $point<T> {
            type Scalar = T;
            type Diff = $vec<T>;
        }
    };
}

// doing this still allows for special additions, like cross products
impl_point!(Point2, Vector2 { x, y });
impl_point!(Point3, Vector3 { x, y, z });
impl_point!(Point4, Vector4 { x, y, z, w });
