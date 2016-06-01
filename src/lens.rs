use super::{Compose, Identity, Lenticuloid};

/// The supertype of all lens families.
pub trait Lens: Lenticuloid {
  fn get(&self, v: Self::InitialSource) -> Self::InitialTarget;

  fn set(&self, v: Self::InitialSource, x: Self::FinalTarget) -> Self::FinalSource {
    self.modify(v, |_| x)
  }

  fn modify<F>(&self, v: Self::InitialSource, f: F) -> Self::FinalSource
    where F: FnOnce(Self::InitialTarget) -> Self::FinalTarget;
}

impl<S, T> Lens for Identity<S, T> {
  fn get(&self, v: S) -> S { v }

  fn set(&self, _v: S, x: T) -> T { x }

  fn modify<F: FnOnce(S) -> T>(&self, v: S, f: F) -> T { f(v) }
}

impl<LF: Lens, LS: Lens + ?Sized> Lens for Compose<LF, LS>
  where LS::InitialTarget: Into<LF::InitialSource>, LF::FinalSource: Into<LS::FinalTarget> {
  fn get(&self, v: Self::InitialSource) -> Self::InitialTarget {
    self.first.get(self.second.get(v).into())
  }

  fn set(&self, v: Self::InitialSource, x: Self::FinalTarget) -> Self::FinalSource {
    self.second.modify(v, |q| self.first.set(q.into(), x).into())
  }

  fn modify<F>(&self, v: Self::InitialSource, f: F) -> Self::FinalSource
    where F: FnOnce(Self::InitialTarget) -> Self::FinalTarget {
    self.second.modify(v, |q| self.first.modify(q.into(), f).into())
  }
}
