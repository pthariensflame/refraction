/// A potentially polymorphic lens family.
pub struct LensFStatic<S, T, A, B, F, G> where F: Fn(S) -> (A, G), G: Fn(B) -> T {
    /// The underlying indexed store comonad coalgebroid.
    pub run: F,
}

pub type LensF<S, T, A, B> = LensFStatic<S, T, A, B, Box<Fn(S) -> (A, Box<Fn(B) -> T>)>;

/// A simple, non-polymorphic lens.
pub type Lens<S, A> = LensF<S, S, A, A>;

impl<S, T, A, B> LensF<S, T, A, B> {
    /// Simple shim for the underlying `run` function.
    pub fn run<'a>(&'a self, v: S) {
        (self.run)(v)
    }

    /// Reading-order lens family composition.
    pub fn and_then<'a, 'b, V, W>(&'a self, other: &'b LensF<A, B, V, W>) -> LensF<S, T, V, W> {
        unimplemented!()
    }

    /// Categorical-order lens family composition.
    pub fn compose<'a, 'b, V, W>(&'a self, other: &'b LensF<V, W, S, T>) -> LensF<V, W, A, B> {
        other.and_then(&self)
    }
}

impl<S, T> LensF<S, T, S, T> {
    /// The identity lens family.
    pub fn id() -> LensF<S, T, S, T> {
        LensF { run: Box::new(move |v| (v, Box::new(move |x| x))) }
    }
}

impl<S, T> Default for LensF<S, T, S, T> {
    fn default() -> LensF<S, T, S, T> {
        LensF::id()
    }
}
