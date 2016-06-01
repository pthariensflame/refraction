use super::{Compose, Lenticuloid};

pub trait ComposeOps<Other: Lenticuloid>: Lenticuloid + Sized {
  fn compose(self, other: Other) -> Compose<Self, Other>
    where Other::InitialTarget: Into<Self::InitialSource>,
          Self::FinalSource: Into<Other::FinalTarget>;

  fn and_then(self, other: Other) -> Compose<Other, Self>
    where Self::InitialTarget: Into<Other::InitialSource>,
          Other::FinalSource: Into<Self::FinalTarget>;
}

impl<This: Lenticuloid, Other: Lenticuloid> ComposeOps<Other> for This {
  fn compose(self, other: Other) -> Compose<Self, Other>
    where Other::InitialTarget: Into<Self::InitialSource>,
          Self::FinalSource: Into<Other::FinalTarget> {
    Compose::of(self, other)
  }

  fn and_then(self, other: Other) -> Compose<Other, Self>
    where Self::InitialTarget: Into<Other::InitialSource>,
          Other::FinalSource: Into<Self::FinalTarget> {
    Compose::of(other, self)
  }
}
