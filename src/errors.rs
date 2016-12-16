//! These lenticuloids deal with Rust's basic error-handling types: `Option`
//! and `Result`.

use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
use super::{Iso, Lens, Lenticuloid, PartialLens, Prism};

/// A lenticuloid that converts between `Option`s and `Result`s in a
/// left-biased fashion.
pub struct OptionFromResultL<A, B, C> {
    phantom_ro: PhantomData<Fn(Result<A, B>) -> Option<A>>,
    phantom_or: PhantomData<Fn(Option<C>) -> Result<C, B>>,
}

impl<A, B, C> OptionFromResultL<A, B, C> {
    #[inline]
    pub fn mk() -> Self {
        OptionFromResultL { phantom_ro: PhantomData,
                            phantom_or: PhantomData, }
    }
}

impl<A, B, C> Debug for OptionFromResultL<A, B, C> {
    fn fmt(&self, fm: &mut Formatter) -> fmt::Result {
        fm.debug_struct("OptionFromResultL")
          .field("phantom_ro", &self.phantom_ro)
          .field("phantom_or", &self.phantom_or)
          .finish()
    }
}

impl<A, B, C> Clone for OptionFromResultL<A, B, C> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        *self = *source;
    }
}

impl<A, B, C> Copy for OptionFromResultL<A, B, C> {}

impl<A, B, C> Default for OptionFromResultL<A, B, C> {
    #[inline]
    fn default() -> Self {
        Self::mk()
    }
}

impl<A, B, C> Lenticuloid for OptionFromResultL<A, B, C> {
    type InitialSource = Result<A, B>;

    type InitialTarget = Option<A>;

    type FinalSource = Result<C, B>;

    type FinalTarget = Option<C>;
}

impl<A, C> PartialLens for OptionFromResultL<A, (), C> {
    fn try_get(&self, v: Self::InitialSource) -> Result<Self::InitialTarget, Self::FinalSource> {
        Ok(match v {
            Ok(a) => Some(a),
            Err(()) => None,
        })
    }

    fn set(&self, v: Self::InitialSource, x: Self::FinalTarget) -> Self::FinalSource {
        match x {
            Some(a) => Ok(a),
            None => Err(()),
        }
    }

    fn modify<F>(&self, v: Self::InitialSource, f: F) -> Self::FinalSource
        where F: FnOnce(Self::InitialTarget) -> Self::FinalTarget
    {
        match f(match v {
            Ok(a) => Some(a),
            Err(()) => None,
        }) {
            Some(b) => Ok(b),
            None => Err(()),
        }
    }

    fn try_modify<F>(&self, v: Self::InitialSource, f: F) -> Self::FinalSource
        where F: FnOnce(Result<Self::InitialTarget, Self::FinalSource>) -> Self::FinalTarget
    {
        match f(Ok(match v {
            Ok(a) => Some(a),
            Err(()) => None,
        })) {
            Some(b) => Ok(b),
            None => Err(()),
        }
    }
}

impl<A, C> Lens for OptionFromResultL<A, (), C> {
    fn get(&self, v: Self::InitialSource) -> Self::InitialTarget {
        match v {
            Ok(a) => Some(a),
            Err(()) => None,
        }
    }
}

impl<A, C> Prism for OptionFromResultL<A, (), C> {
    fn inject(&self, v: Self::FinalTarget) -> Self::FinalSource {
        match v {
            Some(a) => Ok(a),
            None => Err(()),
        }
    }
}

impl<A, C> Iso for OptionFromResultL<A, (), C> {}
