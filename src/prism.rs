use std::ops::Deref;
use super::{Identity, Compose};

/// The supertype of all prism families.
pub trait Prism<S, T, A, B> {
    fn try_get(&self, v: S) -> Result<A, T>;

    fn inject(&self, v: B) -> T;
}

/// The supertype of all simple prisms.
pub trait PrismS<S, A>: Prism<S, S, A, A> {}
impl<S, A, L: Prism<S, S, A, A> + ?Sized> PrismS<S, A> for L {}

impl<S, T> Prism<S, T, S, T> for Identity {
    fn try_get(&self, v: S) -> Result<S, T> {
        Ok(v)
    }

    fn inject(&self, v: T) -> T {
        v
    }
}

impl<S, T, A, B, V, W, LF: Prism<S, T, A, B>, LS: Prism<A, B, V, W> + ?Sized>
    Prism<S, T, V, W> for Compose<LF, A, B, LS> {
	fn try_get(&self, v: S) -> Result<V, T> {
		match self.first.try_get(v) {
			Ok(q) => match self.second.try_get(q) {
				Ok(x) => Ok(x),
				Err(r) => Err(self.first.inject(r)),
			},
			Err(w) => Err(w),
		}
	}

    fn inject(&self, v: W) -> T {
    	self.first.inject(self.second.inject(v))
    }
}

#[derive(Debug)]
pub struct PrismFn<G, H: ?Sized> {
    pub proj: G,
    pub inj: H,
}

impl<G, H: ?Sized> PrismFn<G, H> {
    pub fn fst(&self) -> &G {
        &(self.proj)
    }

    pub fn snd(&self) -> &H {
        &(self.inj)
    }
}

impl<S, T, A, B, G: Deref<Target=Fn(S) -> Result<A, T>>, H: Deref<Target=Fn(B) -> T>>
    Prism<S, T, A, B> for PrismFn<G, H> {
	fn try_get(&self, v: S) -> Result<A, T> {
		(self.fst())(v)
	}

	fn inject(&self, v: B) -> T {
		(self.snd())(v)
	}
}

impl<'l, S, T, A, B, L: Prism<S, T, A, B> + ?Sized> Prism<S, T, A, B> for &'l L {
    fn try_get(&self, v: S) -> Result<A, T> {
        (**self).try_get(v)
    }

    fn inject(&self, v: B) -> T {
        (**self).inject(v)
    }
}

impl<'l, S, T, A, B, L: Prism<S, T, A, B> + ?Sized> Prism<S, T, A, B> for &'l mut L {
    fn try_get(&self, v: S) -> Result<A, T> {
        (**self).try_get(v)
    }

    fn inject(&self, v: B) -> T {
        (**self).inject(v)
    }
}

impl<S, T, A, B, L: Prism<S, T, A, B> + ?Sized> Prism<S, T, A, B> for Box<L> {
    fn try_get(&self, v: S) -> Result<A, T> {
        (**self).try_get(v)
    }

    fn inject(&self, v: B) -> T {
        (**self).inject(v)
    }
}
