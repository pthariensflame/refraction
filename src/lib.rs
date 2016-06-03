//! `refraction` is a lens library for Rust.  Its design is based broadly on
//! that
//! of [the Haskell `lens` package](https://hackage.haskell.org/package/lens),
//! but
//! reworked and reorganized to become more Rusty.
extern crate void;
use std::marker::PhantomData;
use std::fmt::{self, Debug, Formatter};

/// The supertype of all lenticuloids.
pub trait Lenticuloid {
  type InitialSource;

  type InitialTarget;

  type FinalSource;

  type FinalTarget;
}

/// The identity lenticuloid.
pub struct Identity<S, T> {
  phantom_ss: PhantomData<Fn(S) -> S>,
  phantom_tt: PhantomData<Fn(T) -> T>,
}

impl<S, T> Debug for Identity<S, T> {
  fn fmt(&self, fm: &mut Formatter) -> fmt::Result {
    fm.debug_struct("Identity")
      .field("phantom_ss", &self.phantom_ss)
      .field("phantom_tt", &self.phantom_tt)
      .finish()
  }
}

impl<S, T> Clone for Identity<S, T> {
  #[inline]
  fn clone(&self) -> Self { *self }

  #[inline]
  fn clone_from(&mut self, source: &Self) { *self = *source; }
}

impl<S, T> Copy for Identity<S, T> {}

impl<S, T> Default for Identity<S, T> {
  #[inline]
  fn default() -> Self { Self::mk() }
}

impl<S, T> Identity<S, T> {
  #[inline]
  pub fn mk() -> Self {
    Identity {
      phantom_ss: PhantomData,
      phantom_tt: PhantomData,
    }
  }
}

impl<S, T> Lenticuloid for Identity<S, T> {
  type InitialSource = S;

  type InitialTarget = S;

  type FinalSource = T;

  type FinalTarget = T;
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
    Compose {
      first: lf,
      second: ls,
    }
  }
}

impl<LF: Lenticuloid, LS: ?Sized> Lenticuloid for Compose<LF, LS>
  where LS: Lenticuloid<InitialTarget = LF::InitialSource, FinalTarget = LF::FinalSource> {
  type InitialSource = LS::InitialSource;

  type InitialTarget = LF::InitialTarget;

  type FinalSource = LS::FinalSource;

  type FinalTarget = LF::FinalTarget;
}

/// The inversion of a lenticuloid.
#[derive(Clone,Copy,Debug,Default)]
pub struct Invert<L: ?Sized> {
  deinvert: L,
}

impl<L> Invert<L> {
  #[inline]
  pub fn of(l: L) -> Self { Invert { deinvert: l } }
}

impl<L: Lenticuloid + ?Sized> Lenticuloid for Invert<L> {
  type InitialSource = L::FinalTarget;

  type InitialTarget = L::FinalSource;

  type FinalSource = L::InitialTarget;

  type FinalTarget = L::InitialSource;
}

mod partial_lens;
pub use partial_lens::*;

mod lens;
pub use lens::*;

mod prism;
pub use prism::*;

mod iso;
pub use iso::*;

mod ops;
pub use ops::*;

pub mod conv;

pub mod terminal;

pub mod errors;

pub mod access;

pub mod collections;
