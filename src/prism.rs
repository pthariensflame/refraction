use super::{Compose, Identity, Lenticuloid};

/// The supertype of all prism families.
pub trait Prism: Lenticuloid {
  fn try_get(&self, v: Self::InitialSource) -> Result<Self::InitialTarget, Self::FinalSource>;

  fn inject(&self, v: Self::FinalTarget) -> Self::FinalSource;
}

impl<S, T> Prism for Identity<S, T> {
  fn try_get(&self, v: S) -> Result<S, T> { Ok(v) }

  fn inject(&self, v: T) -> T { v }
}

impl<LF: Prism, LS: Prism + ?Sized> Prism for Compose<LF, LS>
  where LS::InitialTarget: Into<LF::InitialSource>, LF::FinalSource: Into<LS::FinalTarget> {
  fn try_get(&self, v: Self::InitialSource) -> Result<Self::InitialTarget, Self::FinalSource> {
    match self.second.try_get(v) {
      Ok(q) => {
        match self.first.try_get(q.into()) {
          Ok(x) => Ok(x),
          Err(r) => Err(self.second.inject(r.into())),
        }
      },
      Err(w) => Err(w),
    }
  }

  fn inject(&self, v: Self::FinalTarget) -> Self::FinalSource {
    self.second.inject(self.first.inject(v).into())
  }
}
