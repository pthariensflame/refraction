use std::ops::Deref;
use std::marker::PhantomData;

/// The type of all lens families.
pub trait Lens<S, T, A, B> {
    fn get(&self, v: S) -> A;
    
    fn set(&self, v: S, x: B) -> T {
    	self.modify(v, |_| x)
    }
    
    fn modify<F: FnOnce(A) -> B>(&self, v: S, f: F) -> T;
}

pub type LensS<S, A> = Lens<S, S, A, A>;

pub struct Identity<S, T> {
	phantom_stst : PhantomData<Fn(S) -> (S, Box<Fn(T) -> T>)>,
}

impl<S, T> Identity<S, T> {
	pub fn mk() -> Self {
		Identity { phantom_stst: PhantomData, }
	}
}

impl<S, T> Clone for Identity<S, T> {
	fn clone(&self) -> Self {
		Self::mk()
	}
}

impl<S, T> Lens<S, T, S, T> for Identity<S, T> {
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

pub struct Compose<S, T, A, B, V, W, LF: Lens<S, T, A, B>, LS: Lens<A, B, V, W>> {
	first: LF,
	second: LS,
	phantom_stab: PhantomData<Fn(S) -> (A, Box<Fn(B) -> T>)>,
	phantom_abvw: PhantomData<Fn(A) -> (V, Box<Fn(W) -> B>)>,
}

impl<S, T, A, B, V, W, LF: Lens<S, T, A, B>, LS: Lens<A, B, V, W>> Compose<S, T, A, B, V, W, LF, LS> {
	pub fn of(f: LF, s: LS) -> Self {
		Compose { first: f, second: s, phantom_stab: PhantomData, phantom_abvw: PhantomData }
	}
}

impl<S, T, A, B, V, W, LF: Lens<S, T, A, B> + Clone, LS: Lens<A, B, V, W> + Clone> Clone for Compose<S, T, A, B, V, W, LF, LS> {
	fn clone(&self) -> Self {
		Self::of(self.first.clone(), self.second.clone())
	}
}

impl<S, T, A, B, V, W, LF: Lens<S, T, A, B>, LS: Lens<A, B, V, W>> Lens<S, T, V, W> for Compose<S, T, A, B, V, W, LF, LS> {
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

impl<S, T, A, B, G: Deref<Target=Fn(B) -> T>> Lens<S, T, A, B> for Fn(S) -> (A, G) {
	fn get(&self, v: S) -> A {
		self(v).0
	}
	
	fn set(&self, v: S, x: B) -> T {
		(self(v).1)(x)
	}
	
	fn modify<F: FnOnce(A) -> B>(&self, v: S, f: F) -> T {
		let (x, g) = self(v);
		g(f(x))
	}
}
