use super::{Compose, Identity, Invert, Lens, Prism};

/// The supertype of all isomorphism families.
pub trait Iso: Lens + Prism
    where Self::AtInitial: Iso,
          Self::AtFinal: Iso
{
}

impl<S, T> Iso for Identity<S, T> {}

impl<LF: Iso, LS: ?Sized> Iso for Compose<LF, LS>
    where LS: Iso<InitialTarget = LF::InitialSource, FinalTarget = LF::FinalSource>,
          LF::AtInitial: Iso,
          LF::AtFinal: Iso,
          LS::AtInitial: Iso,
          LS::AtFinal: Iso
{
}

impl<L: Iso> Iso for Invert<L>
    where L::AtInitial: Iso,
          L::AtFinal: Iso
{
}
