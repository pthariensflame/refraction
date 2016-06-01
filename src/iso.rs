use super::{Compose, Identity, Invert, Lens, Prism};

/// The supertype of all isomorphism families.
pub trait Iso: Lens + Prism {}

impl<S, T> Iso for Identity<S, T> {}

impl<LF: Iso, LS: Iso + ?Sized> Iso for Compose<LF, LS>
  where LS::InitialTarget: Into<LF::InitialSource>, LF::FinalSource: Into<LS::FinalTarget> {
}

impl<L: Iso> Lens for Invert<L> {
  fn get(&self, v: Self::InitialSource) -> Self::InitialTarget { self.deinvert_ref().inject(v) }

  fn set(&self, _v: Self::InitialSource, x: Self::FinalTarget) -> Self::FinalSource {
    self.deinvert_ref().get(x)
  }

  fn modify<F>(&self, v: Self::InitialSource, f: F) -> Self::FinalSource
    where F: FnOnce(Self::InitialTarget) -> Self::FinalTarget {
    let l = self.deinvert_ref();
    l.get(f(l.inject(v)))
  }
}

impl<L: Iso> Prism for Invert<L> {
  fn try_get(&self, v: Self::InitialSource) -> Result<Self::InitialTarget, Self::FinalSource> {
    Ok(self.deinvert_ref().inject(v))
  }

  fn inject(&self, v: Self::FinalTarget) -> Self::FinalSource { self.deinvert_ref().get(v) }
}

impl<L: Iso> Iso for Invert<L> {}
