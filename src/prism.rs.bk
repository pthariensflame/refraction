use super::{Compose, Identity, Invert, Iso, PartialLens};

/// The supertype of all prism families.
pub trait Prism: PartialLens {
  fn inject(&self, v: Self::FinalTarget) -> Self::FinalSource;
}

impl<S, T> Prism for Identity<S, T> {
  #[inline]
  fn inject(&self, v: T) -> T { v }
}

impl<LF: Prism, LS: ?Sized> Prism for Compose<LF, LS>
  where LS: Prism<InitialTarget = LF::InitialSource, FinalTarget = LF::FinalSource> {
  fn inject(&self, v: Self::FinalTarget) -> Self::FinalSource {
    self.second.inject(self.first.inject(v))
  }
}

impl<L: Iso> Prism for Invert<L> {
  #[inline]
  fn inject(&self, v: Self::FinalTarget) -> Self::FinalSource { self.deinvert.get(v) }
}
