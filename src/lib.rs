//! `refraction` is a lens library for Rust.  Its design is based broadly on
//! that of [the Haskell `lens`
//! package](https://hackage.haskell.org/package/lens), but reworked and
//! reorganized to become more Rusty.
//!
//! The `nightly` cargo feature flag can be used to enable some features only
//! available on nightly Rust:
//!
//! - Lenticuloids that deal with the `!` type
//! - `const fn` support
//! - More efficient implementation of some macros
//!
//! To get started easily, import
//! [`refraction::prelude::*`](prelude/index.html).

#![cfg_attr(feature = "nightly", feature(never_type, const_fn))]
#![cfg_attr(feature = "cargo-clippy", allow(expl_impl_clone_on_copy, type_complexity))]

use std::fmt;
use std::marker::PhantomData;
extern crate nodrop;

/// A [prelude module](https://doc.rust-lang.org/std/prelude/#other-preludes)
/// suitible for glob-importing.
pub mod prelude {
    #[doc(no_inline)]
    pub use ::{AndThenExt, ComposeExt, Identity, InvertExt, Iso, Lens, PartialLens, Prism};
}

/// Some utility functions used inside this crate, but possibly useful for
/// others as well.
pub mod util {
    pub fn once_to_mut<'a, X, Y, F>(f_once: F) -> Box<FnMut(X) -> Option<Y> + 'a>
        where F: FnOnce(X) -> Y + 'a
    {
        let mut f_opt: Option<F> = Some(f_once);
        Box::new(move |x| f_opt.take().map(move |f| f(x)))
    }

    pub fn once_to_mut_flatten<'a, X, Y, F>(f_once: F) -> Box<FnMut(X) -> Option<Y> + 'a>
        where F: FnOnce(X) -> Option<Y> + 'a
    {
        let mut f_opt: Option<F> = Some(f_once);
        Box::new(move |x| f_opt.take().and_then(move |f| f(x)))
    }
}

/// The supertype of all lenticuloids.
pub trait Lenticuloid {
    type InitialSource;

    type InitialTarget;

    type FinalSource;

    type FinalTarget;

    type AtInitial: Lenticuloid<InitialSource = Self::InitialSource,
                InitialTarget = Self::InitialTarget,
                FinalSource = Self::InitialSource,
                FinalTarget = Self::InitialTarget,
                AtInitial = Self::AtInitial,
                AtFinal = Self::AtInitial>;

    fn at_initial(&self) -> Self::AtInitial;

    type AtFinal: Lenticuloid<InitialSource = Self::FinalSource,
                InitialTarget = Self::FinalTarget,
                FinalSource = Self::FinalSource,
                FinalTarget = Self::FinalTarget,
                AtInitial = Self::AtFinal,
                AtFinal = Self::AtFinal>;

    fn at_final(&self) -> Self::AtFinal;
}

/// The identity lenticuloid.
pub struct Identity<S, T> {
    phantom_ss: PhantomData<Fn(S) -> S>,
    phantom_tt: PhantomData<Fn(T) -> T>,
}

impl<S, T> fmt::Debug for Identity<S, T> {
    fn fmt(&self, fm: &mut fmt::Formatter) -> fmt::Result {
        fm.debug_struct("Identity")
          .field("phantom_ss", &self.phantom_ss)
          .field("phantom_tt", &self.phantom_tt)
          .finish()
    }
}

impl<S, T> Clone for Identity<S, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        *self = *source;
    }
}

impl<S, T> Copy for Identity<S, T> {}

impl<S, T> Default for Identity<S, T> {
    #[inline]
    fn default() -> Self {
        Self::mk()
    }
}

impl<S, T> Identity<S, T> {
    #[cfg(not(feature = "nightly"))]
    #[inline]
    pub fn mk() -> Self {
        Identity { phantom_ss: PhantomData,
                   phantom_tt: PhantomData, }
    }

    #[cfg(feature = "nightly")]
    #[inline]
    pub const fn mk() -> Self {
        Identity { phantom_ss: PhantomData,
                   phantom_tt: PhantomData, }
    }
}

impl<S, T> Lenticuloid for Identity<S, T> {
    type InitialSource = S;

    type InitialTarget = S;

    type FinalSource = T;

    type FinalTarget = T;

    type AtInitial = Identity<S, S>;

    fn at_initial(&self) -> Self::AtInitial {
        Identity::mk()
    }

    type AtFinal = Identity<T, T>;

    fn at_final(&self) -> Self::AtFinal {
        Identity::mk()
    }
}

/// Composition of lenticuloids.
#[derive(Clone,Copy,Debug,Default)]
pub struct Compose<LF, LS: ?Sized> {
    first: LF,
    second: LS,
}

impl<LF, LS> Compose<LF, LS> {
    #[inline]
    pub fn of(lf: LF, ls: LS) -> Self {
        Compose { first: lf,
                  second: ls, }
    }
}

impl<LF: Lenticuloid, LS: ?Sized> Lenticuloid for Compose<LF, LS>
    where LS: Lenticuloid<InitialTarget = LF::InitialSource, FinalTarget = LF::FinalSource>
{
    type InitialSource = LS::InitialSource;

    type InitialTarget = LF::InitialTarget;

    type FinalSource = LS::FinalSource;

    type FinalTarget = LF::FinalTarget;

    type AtInitial = Compose<LF::AtInitial, LS::AtInitial>;

    fn at_initial(&self) -> Self::AtInitial {
        Compose::of(self.first.at_initial(), self.second.at_initial())
    }

    type AtFinal = Compose<LF::AtFinal, LS::AtFinal>;

    fn at_final(&self) -> Self::AtFinal {
        Compose::of(self.first.at_final(), self.second.at_final())
    }
}

/// The inversion of a lenticuloid.
#[derive(Clone,Copy,Debug,Default)]
pub struct Invert<L: ?Sized> {
    deinvert: L,
}

impl<L> Invert<L> {
    #[inline]
    pub fn of(l: L) -> Self {
        Invert { deinvert: l }
    }
}

impl<L: Lenticuloid + ?Sized> Lenticuloid for Invert<L> {
    type InitialSource = L::FinalTarget;

    type InitialTarget = L::FinalSource;

    type FinalSource = L::InitialTarget;

    type FinalTarget = L::InitialSource;

    type AtInitial = Invert<L::AtFinal>;

    fn at_initial(&self) -> Self::AtInitial {
        Invert::of(self.deinvert.at_final())
    }

    type AtFinal = Invert<L::AtInitial>;

    fn at_final(&self) -> Self::AtFinal {
        Invert::of(self.deinvert.at_initial())
    }
}

mod partial_lens;
pub use partial_lens::*;

mod lens;
pub use lens::*;

mod prism;
pub use prism::*;

mod iso;
pub use iso::*;

#[macro_use]
mod ops;
pub use ops::*;

pub mod access;

pub mod meta;

pub mod terminal;

pub mod conv;

pub mod errors;

pub mod collections;
