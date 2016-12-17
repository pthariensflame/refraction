use super::{Compose, Identity, Invert, Iso, Lenticuloid, util};

pub type Injector<'l, X, Y> = Box<FnMut(X) -> Option<Y> + 'l>;

/// The supertype of all partial lens families.
pub trait PartialLens: Lenticuloid
    where Self::AtInitial: PartialLens,
          Self::AtFinal: PartialLens
{
    fn try_get(&self, v: Self::InitialSource) -> Result<Self::InitialTarget, Self::FinalSource> {
        self.try_get_inject(v).map(|(x, _)| x)
    }

    /// This signature is somewhat hacky; it awaits resolution of the `FnBox`
    /// issue for better design. Notably, the injection function returned by
    /// this method will (if law-abiding) only return `Some` exactly once;
    /// every time afterwards, it will return `None`.
    fn try_get_inject(&self,
                      v: Self::InitialSource)
                      -> Result<(Self::InitialTarget,
                                 Injector<Self::FinalTarget, Self::FinalSource>),
                                Self::FinalSource>;

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
    fn try_get_inject(&self, v: S) -> Result<(S, Injector<T, T>), T> {
        Ok((v, util::once_to_mut(|x| x)))
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
    where LS: PartialLens<InitialTarget = LF::InitialSource, FinalTarget = LF::FinalSource>,
          LF::AtInitial: PartialLens,
          LF::AtFinal: PartialLens,
          LS::AtInitial: PartialLens,
          LS::AtFinal: PartialLens
{
    fn try_get(&self, v: Self::InitialSource) -> Result<Self::InitialTarget, Self::FinalSource> {
        let Compose { first: ref lf, second: ref ls } = *self;
        ls.try_get_inject(v).and_then(move |(q, mut inj)| {
            lf.try_get(q).map_err(move |x| inj(x).unwrap_or_else(|| unreachable!()))
        })
    }

    fn try_get_inject(&self,
                      v: Self::InitialSource)
                      -> Result<(Self::InitialTarget,
                                 Injector<Self::FinalTarget, Self::FinalSource>),
                                Self::FinalSource> {
        let Compose { first: ref lf, second: ref ls } = *self;
        ls.try_get_inject(v).and_then(move |(q, mut inj_q)| {
            let res = lf.try_get_inject(q).map(|(x, mut inj_x)| {
                (x, move |y| inj_x(y).unwrap_or_else(|| unreachable!()))
            });
            match res {
                Ok((x, mut inj_x)) => {
                    Ok((x,
                        util::once_to_mut(move |y| {
                            inj_q(inj_x(y)).unwrap_or_else(|| unreachable!())
                        })))
                }
                Err(q) => Err(inj_q(q).unwrap_or_else(|| unreachable!())),
            }
        })
    }

    fn set(&self, v: Self::InitialSource, x: Self::FinalTarget) -> Self::FinalSource {
        self.second.modify(v, |q| self.first.set(q, x))
    }

    fn exchange(&self,
                v: Self::InitialSource,
                x: Self::FinalTarget)
                -> (Option<Self::InitialTarget>, Self::FinalSource) {
        let (a, b_opt) = self.second.modify_with(v, |q| {
            let (c, d) = self.first.exchange(q, x);
            (d, c)
        });
        (b_opt.and_then(|b| b), a)
    }

    fn modify<F>(&self, v: Self::InitialSource, f: F) -> Self::FinalSource
        where F: FnOnce(Self::InitialTarget) -> Self::FinalTarget
    {
        self.second.modify(v, |q| self.first.modify(q, f))
    }

    fn modify_with<F, X>(&self, v: Self::InitialSource, f: F) -> (Self::FinalSource, Option<X>)
        where F: FnOnce(Self::InitialTarget) -> (Self::FinalTarget, X)
    {
        let (a, b_opt) = self.second.modify_with(v, |q| self.first.modify_with(q, f));
        (a, b_opt.and_then(|b| b))
    }
}

impl<L: Iso> PartialLens for Invert<L>
    where L::AtInitial: Iso,
          L::AtFinal: Iso
{
    #[inline]
    fn try_get(&self, v: Self::InitialSource) -> Result<Self::InitialTarget, Self::FinalSource> {
        Ok(self.deinvert.inject(v))
    }

    #[inline]
    fn try_get_inject(&self,
                      v: Self::InitialSource)
                      -> Result<(Self::InitialTarget,
                                 Injector<Self::FinalTarget, Self::FinalSource>),
                                Self::FinalSource> {
        Ok((self.deinvert.inject(v), util::once_to_mut(move |x| self.deinvert.get(x))))
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
        let l = &self.deinvert;
        (Some(l.inject(v)), l.get(x))
    }

    #[inline]
    fn modify<F>(&self, v: Self::InitialSource, f: F) -> Self::FinalSource
        where F: FnOnce(Self::InitialTarget) -> Self::FinalTarget
    {
        let l = &self.deinvert;
        l.get(f(l.inject(v)))
    }

    #[inline]
    fn modify_with<F, X>(&self, v: Self::InitialSource, f: F) -> (Self::FinalSource, Option<X>)
        where F: FnOnce(Self::InitialTarget) -> (Self::FinalTarget, X)
    {
        let l = &self.deinvert;
        let (x, ret) = f(l.inject(v));
        (l.get(x), Some(ret))
    }
}
