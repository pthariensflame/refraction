use std::ops::*;

/// The type of all lens families.
pub trait Lens<S, T, A, B> {
    type F: Deref<Target = Fn(B) -> T>;
    fn run(&self, v: S) -> (A, Self::F);
}

impl<S, T, A, B, F0: Deref<Target = Fn(B) -> T>> Lens<S, T, A, B, F = F0> {
    pub fn get(&self, v: S) -> A {
        self.run(v).0
    }
    
    pub fn set(&self, v: S, x: B) -> T {
        (self.run(v).1)(x)
    }
    
    pub fn modify<F1: Fn(A) -> B>(&self, v: S, f: F1) -> T {
        let (x, g) = self.run(v);
        g(f(x))
    }
}

pub struct LensRef<'l, S, T, A, B> {
    _run: &'l Fn(S) -> (A, &'l Fn(B) -> T),
}

impl<'l, S, T, A, B> LensRef<'l, S, T, A, B> {
    pub fn new(f: &'l Fn(S) -> (A, &'l Fn(B) -> T)) -> Self {
        LensRef { _run: f }
    }
}

impl<'l, S, T, A, B> Lens<S, T, A, B> for LensRef<'l, S, T, A, B> {
    type F = &'l ((Fn(A) -> B) + 'l);
    fn run(&self, v: S) -> (A, &'l Fn(A) -> B) {
        self._run(v)
    }
}
