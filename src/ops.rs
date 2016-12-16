use super::{Compose, Identity, Invert, Lenticuloid};

/// The identity lenticuloid (function form).
pub fn identity<S, T>() -> Identity<S, T> {
    Identity::mk()
}

/// Extension `trait` for lenticuloid composition in categorical order.
pub trait ComposeExt<Other>: Lenticuloid + Sized
    where Other: Lenticuloid<InitialTarget = Self::InitialSource, FinalTarget = Self::FinalSource>
{
    fn compose(self, other: Other) -> Compose<Self, Other>;
}

impl<This, Other> ComposeExt<Other> for This
    where This: Lenticuloid,
          Other: Lenticuloid<InitialTarget = This::InitialSource, FinalTarget = This::FinalSource>
{
    #[inline]
    fn compose(self, other: Other) -> Compose<Self, Other> {
        Compose::of(self, other)
    }
}

/// Compose all the provided lenticuloids in categorical order.
#[macro_export]
macro_rules! chain_compose {
    () => { $crate::Identity::mk() };
    ($l:expr) => { $l };
    ($lf:expr, $($ls:expr),+) => { Compose::of($lf, chain_compose!($($ls),+)) };
}

/// Extension `trait` for lenticuloid composition in intuitive order.
pub trait AndThenExt<Other: Lenticuloid>
    : Lenticuloid<InitialTarget = Other::InitialSource, FinalTarget = Other::FinalSource> + Sized
    where Other: Lenticuloid
{
    fn and_then(self, other: Other) -> Compose<Other, Self>;
}

impl<This, Other> AndThenExt<Other> for This
    where Other: Lenticuloid,
          This: Lenticuloid<InitialTarget = Other::InitialSource, FinalTarget = Other::FinalSource>
{
    #[inline]
    fn and_then(self, other: Other) -> Compose<Other, Self> {
        Compose::of(other, self)
    }
}

/// Compose all the provided lenticuloids in intuitive order.
#[macro_export]
macro_rules! chain_and_then {
    () => { $crate::Identity::mk() };
    ($l:expr) => { $l };
    ($lf:expr, $($ls:expr),+) => { Compose::of(chain_and_then!($($ls),+), $lf) };
}

/// Extension `trait` for lenticuloid inversion.
pub trait InvertExt: Lenticuloid + Sized {
    fn invert(self) -> Invert<Self>;
}

impl<This: Lenticuloid> InvertExt for This {
    #[inline]
    fn invert(self) -> Invert<Self> {
        Invert::of(self)
    }
}
