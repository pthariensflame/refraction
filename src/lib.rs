//! `refraction` is a lens library for Rust.  Its design is based broadly on that
//! of [the Haskell `lens` package](https://hackage.haskell.org/package/lens), but
//! reworked and reorganized to become more Rusty.
use std::marker::PhantomData;

/// The identity lenticuloid.
#[derive(Debug)]
pub struct Identity<S, T> {
    phantom_ss: PhantomData<Box<Fn(S) -> S>>,
    phantom_tt: PhantomData<Box<Fn(T) -> T>>,
}

impl<S, T> Identity<S, T> {
    pub fn mk() -> Self {
        Identity {
            phantom_ss: PhantomData,
            phantom_tt: PhantomData,
        }
    }
}

impl<S, T> Clone for Identity<S, T> {
    fn clone(&self) -> Self {
        Self::mk()
    }
}

/// Composition of lenticuloids.
#[derive(Debug)]
pub struct Compose<S, T, A, B, V, W, LF, LS: ?Sized> {
    phantom_st: PhantomData<Box<Fn(S) -> T>>,
    phantom_ab: PhantomData<Box<Fn(A) -> B>>,
    phantom_ba: PhantomData<Box<Fn(B) -> A>>,
    phantom_wv: PhantomData<Box<Fn(W) -> V>>,
    first: LF,
    second: LS,
}

impl<S, T, A, B, V, W, LF, LS> Compose<S, T, A, B, V, W, LF, LS> {
    pub fn of(f: LF, s: LS) -> Self {
        Compose {
            phantom_st: PhantomData,
            phantom_ab: PhantomData,
            phantom_ba: PhantomData,
            phantom_wv: PhantomData,
            first: f,
            second: s,
        }
    }
}

impl<S, T, A, B, V, W, LF: Clone, LS: Clone + ?Sized> Clone for Compose<S, T, A, B, V, W, LF, LS> {
    fn clone(&self) -> Self {
        Self::of(self.first.clone(), self.second.clone())
    }
}

/// The first element of a pair.
#[derive(Debug)]
pub struct Fst<A0, A1, B0> {
    phantom_sa: PhantomData<Box<Fn((A0, A1)) -> A0>>,
    phantom_bt: PhantomData<Box<Fn(B0) -> (B0, A1)>>,
}

impl<A0, A1, B0> Fst<A0, A1, B0> {
    pub fn mk() -> Self {
        Fst {
            phantom_sa: PhantomData,
            phantom_bt: PhantomData,
        }
    }
}

impl<A0, A1, B0> Clone for Fst<A0, A1, B0> {
    fn clone(&self) -> Self {
        Self::mk()
    }
}

/// The second element of a pair.
#[derive(Debug)]
pub struct Snd<A0, A1, B1> {
    phantom_sa: PhantomData<Box<Fn((A0, A1)) -> A1>>,
    phantom_bt: PhantomData<Box<Fn(B1) -> (A0, B1)>>,
}

impl<A0, A1, B1> Snd<A0, A1, B1> {
    pub fn mk() -> Self {
        Snd {
            phantom_sa: PhantomData,
            phantom_bt: PhantomData,
        }
    }
}

impl<A0, A1, B1> Clone for Snd<A0, A1, B1> {
    fn clone(&self) -> Self {
        Self::mk()
    }
}

/// The inversion of a lenticuloid.
#[derive(Clone,Copy,Debug)]
pub struct Invert<L: ?Sized>(pub L);

impl<L: ?Sized> Invert<L> {
    pub fn deinvert(&self) -> &L {
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
