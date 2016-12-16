use super::{Compose, Identity, Invert, Iso, Lens, Lenticuloid, Prism};

/// The supertype of all partial lens families.
pub trait PartialLens: Lenticuloid {
    fn try_get(&self, v: Self::InitialSource) -> Result<Self::InitialTarget, Self::FinalSource>;

    fn set(&self, v: Self::InitialSource, x: Self::FinalTarget) -> Self::FinalSource {
        self.modify(v, |_| x)
    }

    fn exchange(&self,
                v: Self::InitialSource,
                x: Self::FinalTarget)
                -> (Option<Self::InitialTarget>, Self::FinalSource) {
        let (a, b) = self.modify_with(v, |y| (x, y));
        (b, a)
    }

    fn modify<F>(&self, v: Self::InitialSource, f: F) -> Self::FinalSource
        where F: FnOnce(Self::InitialTarget) -> Self::FinalTarget
    {
        self.modify_with(v, |x| (f(x), ())).0
    }

    fn modify_with<F, X>(&self, v: Self::InitialSource, f: F) -> (Self::FinalSource, Option<X>)
        where F: FnOnce(Self::InitialTarget) -> (Self::FinalTarget, X);
}

impl<S, T> PartialLens for Identity<S, T> {
    #[inline]
    fn try_get(&self, v: S) -> Result<S, T> {
        Ok(v)
    }

    #[inline]
    fn set(&self, _v: S, x: T) -> T {
        x
    }

    #[inline]
    fn exchange(&self, v: S, x: T) -> (Option<S>, T) {
        (Some(v), x)
    }

    #[inline]
    fn modify<F: FnOnce(S) -> T>(&self, v: S, f: F) -> T {
        f(v)
    }

    #[inline]
    fn modify_with<F: FnOnce(S) -> (T, X), X>(&self, v: S, f: F) -> (T, Option<X>) {
        let (a, b) = f(v);
        (a, Some(b))
    }
}

impl<LF: PartialLens, LS: ?Sized> PartialLens for Compose<LF, LS>
    where LS: PartialLens<InitialTarget = LF::InitialSource, FinalTarget = LF::FinalSource>
{
    fn try_get(&self, v: Self::InitialSource) -> Result<Self::InitialTarget, Self::FinalSource> {
        ()
    }

    fn set(&self, v: Self::InitialSource, x: Self::FinalTarget) -> Self::FinalSource {
        self.second.modify(v, |q| self.first.set(q, x))
    }

    fn exchange(&self,
                v: Self::InitialSource,
                x: Self::FinalTarget)
                -> (Option<Self::InitialTarget>, Self::FinalSource) {
        ()
    }

    fn modify<F>(&self, v: Self::InitialSource, f: F) -> Self::FinalSource
        where F: FnOnce(Self::InitialTarget) -> Self::FinalTarget
    {
        self.second.modify(v, |q| self.first.modify(q, f))
    }

    fn modify_with<F, X>(&self, v: Self::InitialSource, f: F) -> (Self::FinalSource, Option<X>)
        where F: FnOnce(Self::InitialTarget) -> (Self::FinalTarget, X)
    {
        ()
    }
}

impl<L: Iso> PartialLens for Invert<L> {
    #[inline]
    fn try_get(&self, v: Self::InitialSource) -> Result<Self::InitialTarget, Self::FinalSource> {
        Ok(self.deinvert.inject(v))
    }

    #[inline]
    fn set(&self, _v: Self::InitialSource, x: Self::FinalTarget) -> Self::FinalSource {
        self.deinvert.get(x)
    }

    #[inline]
    fn exchange(&self,
                v: Self::InitialSource,
                x: Self::FinalTarget)
                -> (Option<Self::InitialTarget>, Self::FinalSource) {
        ()
    }

    #[inline]
    fn modify<F>(&self, v: Self::InitialSource, f: F) -> Self::FinalSource
        where F: FnOnce(Self::InitialTarget) -> Self::FinalTarget
    {
        let ref l = self.deinvert;
        l.get(f(l.inject(v)))
    }

    #[inline]
    fn modify_with<F, X>(&self, v: Self::InitialSource, f: F) -> (Self::FinalSource, Option<X>)
        where F: FnOnce(Self::InitialTarget) -> (Self::FinalTarget, X)
    {
        let ref l = self.deinvert;
        let (x, ret) = f(l.inject(v));
        (l.get(x), Some(ret))
    }
}
