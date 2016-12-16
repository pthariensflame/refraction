use super::{Compose, Identity, Invert, Iso, Lens, Lenticuloid, Prism, ResultExt};

/// The supertype of all partial lens families.
pub trait PartialLens: Lenticuloid {
    fn try_get(&self, v: Self::InitialSource) -> Result<Self::InitialTarget, Self::FinalSource>;

    fn set(&self, v: Self::InitialSource, x: Self::FinalTarget) -> Self::FinalSource {
        self.modify(v, |_| x)
    }

    fn modify<F>(&self, v: Self::InitialSource, f: F) -> Self::FinalSource
        where F: FnOnce(Self::InitialTarget) -> Self::FinalTarget;
}

impl<S, T> PartialLens for Identity<S, T> {
    #[inline]
    fn try_get(&self, v: S) -> Result<S, T> {
        Ok(v)
    }

    #[inline]
    fn set(&self, _v: S, x: T) -> T {
        x
    }

    #[inline]
    fn modify<F: FnOnce(S) -> T>(&self, v: S, f: F) -> T {
        f(v)
    }
}

impl<LF: PartialLens, LS: ?Sized> PartialLens for Compose<LF, LS>
    where LS: PartialLens<InitialTarget = LF::InitialSource, FinalTarget = LF::FinalSource>
{
    fn try_get(&self, v: Self::InitialSource) -> Result<Self::InitialTarget, Self::FinalSource> {
        match self.second.try_get(v) {
            Ok(q) => match self.first.try_get(q) {
            },
            Err(w) => Err(w),
        }
    }

    fn set(&self, v: Self::InitialSource, x: Self::FinalTarget) -> Self::FinalSource {
        self.second.modify(v, |q| self.first.set(q, x))
    }

    fn modify<F>(&self, v: Self::InitialSource, f: F) -> Self::FinalSource
        where F: FnOnce(Self::InitialTarget) -> Self::FinalTarget
    {
        self.second.modify(v, |q| self.first.modify(q, f))
    }
}

impl<L: Iso> PartialLens for Invert<L> {
    #[inline]
    fn try_get(&self, v: Self::InitialSource) -> Result<Self::InitialTarget, Self::FinalSource> {
        Ok(self.deinvert.inject(v))
    }

    #[inline]
    fn set(&self, _v: Self::InitialSource, x: Self::FinalTarget) -> Self::FinalSource {
        self.deinvert.get(x)
    }

    #[inline]
    fn modify<F>(&self, v: Self::InitialSource, f: F) -> Self::FinalSource
        where F: FnOnce(Self::InitialTarget) -> Self::FinalTarget
    {
        let ref l = self.deinvert;
        l.get(f(l.inject(v)))
    }
}
