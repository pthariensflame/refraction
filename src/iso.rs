use super::{Compose, Conv, Identity, Invert, Lens, Prism};

/// The supertype of all isomorphism families.
pub trait Iso: Lens + Prism {}

impl<S, T> Iso for Identity<S, T> {}

impl<LF: Iso, LS: ?Sized> Iso for Compose<LF, LS>
  where LS: Iso<InitialTarget = LF::InitialSource, FinalTarget = LF::FinalSource> {
}

impl<L: Iso> Iso for Invert<L> {}

impl<S, A, T, B> Iso for Conv<S, A, T, B> where S: Into<A>, B: Into<T> {}
