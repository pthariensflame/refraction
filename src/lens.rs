use super::{Compose, Identity, Invert, Iso, PartialLens};

/// The supertype of all lens families.
pub trait Lens: PartialLens {
    fn get(&self, v: Self::InitialSource) -> Self::InitialTarget;
}

impl<S, T> Lens for Identity<S, T> {
    #[inline]
    fn get(&self, v: S) -> S {
        v
    }
}

impl<LF: Lens, LS: ?Sized> Lens for Compose<LF, LS>
    where LS: Lens<InitialTarget = LF::InitialSource, FinalTarget = LF::FinalSource>
{
    fn get(&self, v: Self::InitialSource) -> Self::InitialTarget {
        self.first.get(self.second.get(v))
    }
}

impl<L: Iso> Lens for Invert<L> {
    #[inline]
    fn get(&self, v: Self::InitialSource) -> Self::InitialTarget {
        self.deinvert.inject(v)
    }
}
