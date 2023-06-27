use crate::{Alias, ExpandAlias, Isosurface, LiftAdt, Point, IntoMonad};

use t_funk::{macros::{applicative::Applicative, functor::Functor, monad::Monad}, typeclass::monad::Identity};

// Circle field symbol
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Circle<T>(pub T);

impl<T> IntoMonad for Circle<T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T> LiftAdt for Circle<T> {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<T, D> ExpandAlias<D> for Circle<T> {
    type ExpandAlias = (Point, Isosurface<T>);

    fn expand_alias(self) -> Self::ExpandAlias {
        (Point, Isosurface(self.0))
    }
}
