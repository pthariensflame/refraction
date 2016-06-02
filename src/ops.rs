use super::{Compose, Invert, Lenticuloid};

/// Extension `trait` for lenticuloid composition (in either order).
pub trait ComposeExt<Other>: Lenticuloid + Sized
  where Other: Lenticuloid<InitialTarget = Self::InitialSource, FinalTarget = Self::FinalSource> {
  fn compose(self, other: Other) -> Compose<Self, Other>;
}

pub trait AndThenExt<Other: Lenticuloid>
  : Lenticuloid<InitialTarget = Other::InitialSource, FinalTarget = Other::FinalSource> + Sized
  where Other: Lenticuloid {
  fn and_then(self, other: Other) -> Compose<Other, Self>;
}


impl<This, Other> ComposeExt<Other> for This
  where This: Lenticuloid,
        Other: Lenticuloid<InitialTarget = This::InitialSource, FinalTarget = This::FinalSource> {
  fn compose(self, other: Other) -> Compose<Self, Other> { Compose::of(self, other) }
}

impl<This, Other> AndThenExt<Other> for This
  where Other: Lenticuloid,
        This: Lenticuloid<InitialTarget = Other::InitialSource, FinalTarget = Other::FinalSource> {
  fn and_then(self, other: Other) -> Compose<Other, Self> { Compose::of(other, self) }
}

/// Extension trait for lenticuloid inversion.
pub trait InvertExt: Lenticuloid + Sized {
  fn invert(self) -> Invert<Self>;
}

impl<This: Lenticuloid> InvertExt for This {
  fn invert(self) -> Invert<Self> { Invert::of(self) }
}
