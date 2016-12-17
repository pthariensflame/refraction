use super::{Compose, Identity, Invert, Iso, PartialLens};

/// The supertype of all lens families.
pub trait Lens: PartialLens
    where Self::AtInitial: Lens,
          Self::AtFinal: Lens
{
    fn get(&self, v: Self::InitialSource) -> Self::InitialTarget;
}

impl<S, T> Lens for Identity<S, T> {
    #[inline]
    fn get(&self, v: S) -> S {
        v
    }
}

impl<LF: Lens, LS: ?Sized> Lens for Compose<LF, LS>
    where LS: Lens<InitialTarget = LF::InitialSource, FinalTarget = LF::FinalSource>,
          LF::AtInitial: Lens,
          LF::AtFinal: Lens,
          LS::AtInitial: Lens,
          LS::AtFinal: Lens
{
    fn get(&self, v: Self::InitialSource) -> Self::InitialTarget {
        self.first.get(self.second.get(v))
    }
}

impl<L: Iso> Lens for Invert<L>
    where L::AtInitial: Iso,
          L::AtFinal: Iso
{
    #[inline]
    fn get(&self, v: Self::InitialSource) -> Self::InitialTarget {
        self.deinvert.inject(v)
    }
}
