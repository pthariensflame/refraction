use std::ops::Deref;
use super::{Compose, Identity, Invert, Iso};

mod aux {
  pub trait SameAux<Other: ?Sized>
    where Other: SameAux<Self> {
    fn witness_aux(&self) -> &Other;
  }

  impl<This: ?Sized> SameAux<This> for This {
    #[inline]
    fn witness_aux(&self) -> &This { self }
  }
}

/// A helper trait that guarantees that the types `Self` and `Other` are
/// strictly identical.  It uses an auxiliarly hidden trait (`SameAux`) to
/// accomplish this in such a way that `Same` cannot be implemented for
/// anything other than the reflexive case.
///
/// When using this trait to specify that two types (say, `A` and `B`) are identical, you simply include `A: Same<B>` and `B: Same<A>` in you `where` clause, then use the `witness` and `
///
/// Generic `impl`s
/// ---
///
/// The (helper-encoded) reflexive case:
///
/// ```ignore
/// impl<This: ?Sized, Other: ?Sized> Same<Other> for This
///   where This: SameAux<Other>, Other: SameAux<This> {}
/// ```
pub trait Same<Other: ?Sized>: aux::SameAux<Other>
  where Other: Same<Self> {
  /// A witness to the equality of
  fn witness(&self) -> &Other;
}

impl<This: ?Sized, Other: ?Sized> Same<Other> for This
  where This: aux::SameAux<Other>, Other: aux::SameAux<This> {
  #[inline]
  fn witness(&self) -> &Other { self.witness_aux() }
}

/// The supertype of all equality witness families.
pub trait Equality<This: Iso + Same<Self> + ?Sized = Self>: Iso + Same<This>
  where Self::InitialSource: Same<This::InitialTarget>, Self::FinalSource: Same<This::FinalTarget>,
        Self::InitialTarget: Same<This::InitialSource>, Self::FinalTarget: Same<This::FinalSource>,
        This::InitialSource: Same<Self::InitialTarget>, This::FinalSource: Same<Self::FinalTarget>,
        This::InitialTarget: Same<Self::InitialSource>, This::FinalTarget: Same<Self::FinalSource> {
}

impl<S, T> Equality for Identity<S, T> {}
