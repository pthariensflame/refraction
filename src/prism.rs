use super::{Compose, Identity, Invert, Iso, Lenticuloid};

/// The supertype of all prism families.
pub trait Prism: Lenticuloid {
  fn try_get(&self, v: Self::InitialSource) -> Result<Self::InitialTarget, Self::FinalSource>;

  fn inject(&self, v: Self::FinalTarget) -> Self::FinalSource;
}

impl<S, T> Prism for Identity<S, T> {
  #[inline]
  fn try_get(&self, v: S) -> Result<S, T> { Ok(v) }

  #[inline]
  fn inject(&self, v: T) -> T { v }
}

impl<LF: Prism, LS: ?Sized> Prism for Compose<LF, LS>
  where LS: Prism<InitialTarget = LF::InitialSource, FinalTarget = LF::FinalSource> {
  fn try_get(&self, v: Self::InitialSource) -> Result<Self::InitialTarget, Self::FinalSource> {
    match self.second.try_get(v) {
      Ok(q) => {
        match self.first.try_get(q) {
          Ok(x) => Ok(x),
          Err(r) => Err(self.second.inject(r)),
        }
      },
      Err(w) => Err(w),
    }
  }

  fn inject(&self, v: Self::FinalTarget) -> Self::FinalSource {
    self.second.inject(self.first.inject(v))
  }
}

impl<L: Iso> Prism for Invert<L> {
  #[inline]
  fn try_get(&self, v: Self::InitialSource) -> Result<Self::InitialTarget, Self::FinalSource> {
    Ok(self.deinvert.inject(v))
  }

  #[inline]
  fn inject(&self, v: Self::FinalTarget) -> Self::FinalSource { self.deinvert.get(v) }
}
