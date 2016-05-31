//! `refraction` is a lens library for Rust.  Its design is based broadly on that
//! of [the Haskell `lens` package](https://hackage.haskell.org/package/lens), but
//! reworked and reorganized to become more Rusty.
use std::marker::PhantomData;

/// The identity lenticuloid.
#[derive(Clone,Copy,Debug)]
pub struct Identity;

impl Identity {
    pub fn mk() -> Self {
        Identity
    }
}

/// Composition of lenticuloids.
#[derive(Clone,Copy,Debug)]
pub struct Compose<LF, A: ?Sized, B: ?Sized, LS: ?Sized> {
    first: LF,
    phantom_ab: PhantomData<Fn(A) -> B>,
    phantom_ba: PhantomData<Fn(B) -> A>,
    second: LS,
}

impl<LF, A: ?Sized, B: ?Sized, LS> Compose<LF, A, B, LS> {
    pub fn of(f: LF, s: LS) -> Self {
        Compose {
            first: f,
            phantom_ab: PhantomData,
            phantom_ba: PhantomData,
            second: s,
        }
    }
}

/// The first element of a pair.
#[derive(Clone,Copy,Debug)]
pub struct Fst;

impl Fst {
    pub fn mk() -> Self {
        Fst
    }
}

/// The second element of a pair.
#[derive(Clone,Copy,Debug)]
pub struct Snd;

impl Snd {
    pub fn mk() -> Self {
        Snd
    }
}

/// The inversion of a lenticuloid.
#[derive(Clone,Copy,Debug)]
pub struct Invert<L: ?Sized>(L);

impl<L: ?Sized> Invert<L> {
    fn deinvert_ref(&self) -> &L {
        let &Invert(ref res) = self;
        res
    }
}

mod lens;
pub use lens::*;

mod prism;
pub use prism::*;

mod iso;
pub use iso::*;
