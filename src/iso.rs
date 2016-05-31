use std::ops::Deref;
use super::{Lens, LensS, Prism, PrismS, Identity, Compose, Invert};

/// The supertype of all isomorphism families.
pub trait Iso<S, T, A, B>: Lens<S, T, A, B> + Prism<S, T, A, B> {}

/// The supertype of all simple isomorphisms.
pub trait IsoS<S, A>: Iso<S, S, A, A> + LensS<S, A> + PrismS<S, A> {}
impl<S, A, L: Iso<S, S, A, A> + ?Sized> IsoS<S, A> for L {}

impl<S, T> Iso<S, T, S, T> for Identity<S, T> {}

impl<S, T, A, B, V, W, LF: Iso<S, T, A, B>, LS: Iso<A, B, V, W> + ?Sized>
    Iso<S, T, V, W> for Compose<S, T, A, B, V, W, LF, LS> {}

impl<B, A, T, S, L: Iso<B, A, T, S>> Lens<S, T, A, B> for Invert<L> {
    fn get(&self, v: S) -> A {
        self.deinvert().inject(v)
    }

    fn set(&self, _v: S, x: B) -> T {
        self.deinvert().get(x)
    }

    fn modify<F: FnOnce(A) -> B>(&self, v: S, f: F) -> T {
        let l = self.deinvert();
        l.get(f(l.inject(v)))
    }
}

impl<B, A, T, S, L: Iso<B, A, T, S>> Prism<S, T, A, B> for Invert<L> {
    fn try_get(&self, v: S) -> Result<A, T> {
        Ok(self.deinvert().inject(v))
    }

    fn inject(&self, v: B) -> T {
        self.deinvert().get(v)
    }
}

impl<B, A, T, S, L: Iso<B, A, T, S>> Iso<S, T, A, B> for Invert<L> {}

#[derive(Debug)]
pub struct IsoFn<G, H: ?Sized> {
    pub proj: G,
    pub inj: H,
}

impl<G, H: ?Sized> IsoFn<G, H> {
    pub fn fst(&self) -> &G {
        &(self.proj)
    }

    pub fn snd(&self) -> &H {
        &(self.inj)
    }
}


impl<S, T, A, B, G: Deref<Target=Fn(S) -> A>, H: Deref<Target=Fn(B) -> T> + ?Sized>
    Lens<S, T, A, B> for IsoFn<G, H> {
	fn get(&self, v: S) -> A {
		(self.fst())(v)
	}

    fn set(&self, _v: S, x: B) -> T {
    	(self.snd())(x)
    }

    fn modify<F: FnOnce(A) -> B>(&self, v: S, f: F) -> T {
    	(self.snd())(f((self.fst())(v)))
    }
}

impl<S, T, A, B, G: Deref<Target=Fn(S) -> A>, H: Deref<Target=Fn(B) -> T>>
    Prism<S, T, A, B> for IsoFn<G, H> {
	fn try_get(&self, v: S) -> Result<A, T> {
		Ok((self.fst())(v))
	}

	fn inject(&self, v: B) -> T {
		(self.snd())(v)
	}
}

impl<S, T, A, B, G: Deref<Target=Fn(S) -> A>, H: Deref<Target=Fn(B) -> T>>
    Iso<S, T, A, B> for IsoFn<G, H> {}

impl<'l, S, T, A, B, L: Iso<S, T, A, B> + ?Sized> Iso<S, T, A, B> for &'l L {}

impl<'l, S, T, A, B, L: Iso<S, T, A, B> + ?Sized> Iso<S, T, A, B> for &'l mut L {}

impl<S, T, A, B, L: Iso<S, T, A, B> + ?Sized> Iso<S, T, A, B> for Box<L> {}
