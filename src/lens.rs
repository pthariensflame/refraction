use super::{Compose, Conv, Identity, Invert, Iso, Lenticuloid};

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

impl<LF: Lens, LS: ?Sized> Lens for Compose<LF, LS>
  where LS: Lens<InitialTarget = LF::InitialSource, FinalTarget = LF::FinalSource> {
  fn get(&self, v: Self::InitialSource) -> Self::InitialTarget {
    self.first.get(self.second.get(v))
  }

  fn set(&self, v: Self::InitialSource, x: Self::FinalTarget) -> Self::FinalSource {
    self.second.modify(v, |q| self.first.set(q, x))
  }

  fn modify<F>(&self, v: Self::InitialSource, f: F) -> Self::FinalSource
    where F: FnOnce(Self::InitialTarget) -> Self::FinalTarget {
    self.second.modify(v, |q| self.first.modify(q, f))
  }
}

impl<L: Iso> Lens for Invert<L> {
  fn get(&self, v: Self::InitialSource) -> Self::InitialTarget { self.deinvert.inject(v) }

  fn set(&self, _v: Self::InitialSource, x: Self::FinalTarget) -> Self::FinalSource {
    self.deinvert.get(x)
  }

  fn modify<F>(&self, v: Self::InitialSource, f: F) -> Self::FinalSource
    where F: FnOnce(Self::InitialTarget) -> Self::FinalTarget {
    let ref l = self.deinvert;
    l.get(f(l.inject(v)))
  }
}

impl<S, A, T, B> Lens for Conv<S, A, T, B>
  where S: Into<A>, B: Into<T> {
  fn get(&self, v: Self::InitialSource) -> Self::InitialTarget { v.into() }

  fn set(&self, _v: Self::InitialSource, x: Self::FinalTarget) -> Self::FinalSource { x.into() }

  fn modify<F>(&self, v: Self::InitialSource, f: F) -> Self::FinalSource
    where F: FnOnce(Self::InitialTarget) -> Self::FinalTarget {
    f(v.into()).into()
  }
}
