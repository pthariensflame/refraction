use std::ops::Deref;
use std::marker::PhantomData;
use super::{Identity, Compose, Fst, Snd};

/// The supertype of all lens families.
pub trait Lens<S, T, A, B> {
    fn get(&self, v: S) -> A;

    fn set(&self, v: S, x: B) -> T {
        self.modify(v, |_| x)
    }

    fn modify<F: FnOnce(A) -> B>(&self, v: S, f: F) -> T;
}

/// The supertype of all simple lenses.
pub trait LensS<S, A>: Lens<S, S, A, A> {}
impl<S, A, L: Lens<S, S, A, A> + ?Sized> LensS<S, A> for L {}

impl<S, T> Lens<S, T, S, T> for Identity {
    fn get(&self, v: S) -> S {
        v
    }

    fn set(&self, _v: S, x: T) -> T {
        x
    }

    fn modify<F: FnOnce(S) -> T>(&self, v: S, f: F) -> T {
        f(v)
    }
}

impl<S, T, A, B, V, W, LF: Lens<S, T, A, B>, LS: Lens<A, B, V, W> + ?Sized>
    Lens<S, T, V, W> for Compose<LF, A, B, LS> {
    fn get(&self, v: S) -> V {
    	self.second.get(self.first.get(v))
    }

    fn set(&self, v: S, x: W) -> T {
    	self.first.modify(v, |q| self.second.set(q, x))
    }

    fn modify<F: FnOnce(V) -> W>(&self, v: S, f: F) -> T {
    	self.first.modify(v, |q| self.second.modify(q, f))
    }
}

#[derive(Clone,Copy,Debug)]
pub struct LensFn<S, A, H, G: Fn(S) -> (A, H) + ?Sized> {
    phantom_g: PhantomData<Fn(S) -> (A, H)>,
    pub underlying: G,
}

impl<S, A, H, G: Fn(S) -> (A, H) + ?Sized> LensFn<S, A, H, G> {
    pub fn new(f: G) -> Self
        where G: Sized
    {
        LensFn {
            phantom_g: PhantomData,
            underlying: f,
        }
    }

    pub fn invoke(&self, v: S) -> (A, H) {
        let ref f = self.underlying;
        f(v)
    }
}

impl<S, T, A, B, H: Deref<Target=Fn(B) -> T>, G: Fn(S) -> (A, H) + ?Sized>
    Lens<S, T, A, B> for LensFn<S, A, H, G> {
	fn get(&self, v: S) -> A {
		self.invoke(v).0
	}

	fn set(&self, v: S, x: B) -> T {
		(self.invoke(v).1)(x)
	}

	fn modify<F: FnOnce(A) -> B>(&self, v: S, f: F) -> T {
		let t = self.invoke(v);
		(t.1)(f(t.0))
	}
}

impl<'l, S, T, A, B, L: Lens<S, T, A, B> + ?Sized> Lens<S, T, A, B> for &'l L {
    fn get(&self, v: S) -> A {
        (**self).get(v)
    }

    fn set(&self, v: S, x: B) -> T {
        (**self).set(v, x)
    }

    fn modify<F: FnOnce(A) -> B>(&self, v: S, f: F) -> T {
        (**self).modify(v, f)
    }
}

impl<'l, S, T, A, B, L: Lens<S, T, A, B> + ?Sized> Lens<S, T, A, B> for &'l mut L {
    fn get(&self, v: S) -> A {
        (**self).get(v)
    }

    fn set(&self, v: S, x: B) -> T {
        (**self).set(v, x)
    }

    fn modify<F: FnOnce(A) -> B>(&self, v: S, f: F) -> T {
        (**self).modify(v, f)
    }
}

impl<S, T, A, B, L: Lens<S, T, A, B> + ?Sized> Lens<S, T, A, B> for Box<L> {
    fn get(&self, v: S) -> A {
        (**self).get(v)
    }

    fn set(&self, v: S, x: B) -> T {
        (**self).set(v, x)
    }

    fn modify<F: FnOnce(A) -> B>(&self, v: S, f: F) -> T {
        (**self).modify(v, f)
    }
}

impl<A0, A1, B0> Lens<(A0, A1), (B0, A1), A0, B0> for Fst {
    fn get(&self, v: (A0, A1)) -> A0 {
        v.0
    }

    fn set(&self, v: (A0, A1), x: B0) -> (B0, A1) {
        (x, v.1)
    }

    fn modify<F: FnOnce(A0) -> B0>(&self, v: (A0, A1), f: F) -> (B0, A1) {
        (f(v.0), v.1)
    }
}

impl<A0, A1, B1> Lens<(A0, A1), (A0, B1), A1, B1> for Snd {
    fn get(&self, v: (A0, A1)) -> A1 {
        v.1
    }

    fn set(&self, v: (A0, A1), x: B1) -> (A0, B1) {
        (v.0, x)
    }

    fn modify<F: FnOnce(A1) -> B1>(&self, v: (A0, A1), f: F) -> (A0, B1) {
        (v.0, f(v.1))
    }
}
