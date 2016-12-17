//! These lenticuloids deal with the trivial and the impossible; that is, they
//! operate with values of types `()` and `!` (the latter only on nightly Rust,
//! with the `nightly` feature flag enabled).

use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
use super::{Lens, Lenticuloid, PartialLens, Prism};

#[cfg(feature = "nightly")]
/// A `Lens` to "extract" anything from `!`.
pub struct FromNever<A, B> {
    phantom_na: PhantomData<Fn(!) -> A>,
    phantom_bn: PhantomData<Fn(B) -> !>,
}

#[cfg(feature = "nightly")]
impl<A, B> FromNever<A, B> {
    pub const fn mk() -> Self {
        FromNever { phantom_na: PhantomData,
                    phantom_bn: PhantomData, }
    }
}

#[cfg(feature = "nightly")]
impl<A, B> Debug for FromNever<A, B> {
    fn fmt(&self, fm: &mut Formatter) -> fmt::Result {
        fm.debug_struct("FromNever")
          .field("phantom_na", &self.phantom_na)
          .field("phantom_bn", &self.phantom_bn)
          .finish()
    }
}

#[cfg(feature = "nightly")]
impl<A, B> Clone for FromNever<A, B> {
    fn clone(&self) -> Self {
        *self
    }

    fn clone_from(&mut self, source: &Self) {
        *self = *source;
    }
}

#[cfg(feature = "nightly")]
impl<A, B> Copy for FromNever<A, B> {}

#[cfg(feature = "nightly")]
impl<A, B> Default for FromNever<A, B> {
    fn default() -> Self {
        Self::mk()
    }
}

#[cfg(feature = "nightly")]
impl<A, B> Lenticuloid for FromNever<A, B> {
    type InitialSource = !;

    type InitialTarget = A;

    type FinalSource = !;

    type FinalTarget = B;

    type AtInitial = FromNever<A, A>;

    fn at_initial(&self) -> Self::AtInitial {
        FromNever::mk()
    }

    type AtFinal = FromNever<B, B>;

    fn at_final(&self) -> Self::AtFinal {
        FromNever::mk()
    }
}

#[cfg(feature = "nightly")]
impl<A, B> PartialLens for FromNever<A, B> {
    fn try_get(&self, v: !) -> Result<A, !> {
        v
    }

    fn set(&self, v: !, _x: B) -> ! {
        v
    }

    fn exchange(&self, v: !, _x: B) -> (Option<A>, !) {
        v
    }

    fn modify<F: FnOnce(A) -> B>(&self, v: !, _f: F) -> ! {
        v
    }

    fn modify_with<F, X>(&self, v: !, f: F) -> (!, Option<X>)
        where F: FnOnce(A) -> (B, X)
    {
        v
    }
}

#[cfg(feature = "nightly")]
impl<A, B> Lens for FromNever<A, B> {
    fn get(&self, v: !) -> A {
        v
    }
}

#[cfg(feature = "nightly")]
/// A `Prism` to "inject" `!` into anything.
pub struct ToNever<S> {
    phantom_ns: PhantomData<Fn(!) -> S>,
    phantom_sn: PhantomData<Fn(S) -> !>,
}

#[cfg(feature = "nightly")]
impl<S> ToNever<S> {
    pub const fn mk() -> Self {
        ToNever { phantom_ns: PhantomData,
                  phantom_sn: PhantomData, }
    }
}

#[cfg(feature = "nightly")]
impl<S> Debug for ToNever<S> {
    fn fmt(&self, fm: &mut Formatter) -> fmt::Result {
        fm.debug_struct("ToNever")
          .field("phantom_ns", &self.phantom_ns)
          .field("phantom_sn", &self.phantom_sn)
          .finish()
    }
}

#[cfg(feature = "nightly")]
impl<S> Clone for ToNever<S> {
    fn clone(&self) -> Self {
        *self
    }

    fn clone_from(&mut self, source: &Self) {
        *self = *source;
    }
}

#[cfg(feature = "nightly")]
impl<S> Copy for ToNever<S> {}

#[cfg(feature = "nightly")]
impl<S> Default for ToNever<S> {
    fn default() -> Self {
        Self::mk()
    }
}

#[cfg(feature = "nightly")]
impl<S> Lenticuloid for ToNever<S> {
    type InitialSource = S;

    type InitialTarget = !;

    type FinalSource = S;

    type FinalTarget = !;

    type AtInitial = ToNever<S>;

    fn at_initial(&self) -> Self::AtInitial {
        ToNever::mk()
    }

    type AtFinal = ToNever<S>;

    fn at_final(&self) -> Self::AtFinal {
        ToNever::mk()
    }
}

#[cfg(feature = "nightly")]
impl<S> PartialLens for ToNever<S> {
    fn try_get(&self, v: S) -> Result<!, S> {
        Err(v)
    }

    fn set(&self, _v: S, x: !) -> S {
        x
    }

    fn exchange(&self, _v: S, x: !) -> (Option<!>, S) {
        x
    }

    fn modify<F>(&self, v: S, _f: F) -> S
        where F: FnOnce(!) -> !
    {
        v
    }

    fn modify_with<F, X>(&self, v: S, f: F) -> (S, Option<X>)
        where F: FnOnce(!) -> (!, X)
    {
        (v, None)
    }
}

#[cfg(feature = "nightly")]
impl<S> Prism for ToNever<S> {
    fn inject(&self, v: !) -> S {
        v
    }
}

/// A `Prism` to "inject" anything into `()`.
pub struct FromUnit<A, B> {
    phantom_ua: PhantomData<Fn(()) -> A>,
    phantom_bu: PhantomData<Fn(B) -> ()>,
}

impl<A, B> FromUnit<A, B> {
    #[cfg(not(feature = "nightly"))]
    pub fn mk() -> Self {
        FromUnit { phantom_ua: PhantomData,
                   phantom_bu: PhantomData, }
    }

    #[cfg(feature = "nightly")]
    pub const fn mk() -> Self {
        FromUnit { phantom_ua: PhantomData,
                   phantom_bu: PhantomData, }
    }
}

impl<A, B> Debug for FromUnit<A, B> {
    fn fmt(&self, fm: &mut Formatter) -> fmt::Result {
        fm.debug_struct("FromUnit")
          .field("phantom_ua", &self.phantom_ua)
          .field("phantom_bu", &self.phantom_bu)
          .finish()
    }
}

impl<A, B> Clone for FromUnit<A, B> {
    fn clone(&self) -> Self {
        *self
    }

    fn clone_from(&mut self, source: &Self) {
        *self = *source;
    }
}

impl<A, B> Copy for FromUnit<A, B> {}

impl<A, B> Default for FromUnit<A, B> {
    fn default() -> Self {
        Self::mk()
    }
}

impl<A, B> Lenticuloid for FromUnit<A, B> {
    type InitialSource = ();

    type InitialTarget = A;

    type FinalSource = ();

    type FinalTarget = B;

    type AtInitial = FromUnit<A, A>;

    fn at_initial(&self) -> Self::AtInitial {
        FromUnit::mk()
    }

    type AtFinal = FromUnit<B, B>;

    fn at_final(&self) -> Self::AtFinal {
        FromUnit::mk()
    }
}

impl<A, B> PartialLens for FromUnit<A, B> {
    fn try_get(&self, v: ()) -> Result<A, ()> {
        Err(v)
    }

    fn set(&self, v: (), _x: B) -> () {
        v
    }

    fn exchange(&self, v: (), x: B) -> (Option<A>, ()) {
        (None, ())
    }

    fn modify<F>(&self, v: (), _f: F) -> ()
        where F: FnOnce(A) -> B
    {
        v
    }

    fn modify_with<F, X>(&self, v: (), f: F) -> ((), Option<X>)
        where F: FnOnce(A) -> (B, X)
    {
        ((), None)
    }
}

impl<A, B> Prism for FromUnit<A, B> {
    fn inject(&self, _v: B) -> () {
        ()
    }
}

/// A `Lens` to "extract" `()` from anything.
pub struct ToUnit<S> {
    phantom_su: PhantomData<Fn(S) -> ()>,
    phantom_us: PhantomData<Fn(()) -> S>,
}

impl<S> ToUnit<S> {
    #[cfg(not(feature = "nightly"))]
    pub fn mk() -> Self {
        ToUnit { phantom_su: PhantomData,
                 phantom_us: PhantomData, }
    }

    #[cfg(feature = "nightly")]
    pub const fn mk() -> Self {
        ToUnit { phantom_su: PhantomData,
                 phantom_us: PhantomData, }
    }
}

impl<S> Debug for ToUnit<S> {
    fn fmt(&self, fm: &mut Formatter) -> fmt::Result {
        fm.debug_struct("ToUnit")
          .field("phantom_su", &self.phantom_su)
          .field("phantom_us", &self.phantom_us)
          .finish()
    }
}

impl<S> Clone for ToUnit<S> {
    fn clone(&self) -> Self {
        *self
    }

    fn clone_from(&mut self, source: &Self) {
        *self = *source;
    }
}

impl<S> Copy for ToUnit<S> {}

impl<S> Default for ToUnit<S> {
    fn default() -> Self {
        Self::mk()
    }
}

impl<S> Lenticuloid for ToUnit<S> {
    type InitialSource = S;

    type InitialTarget = ();

    type FinalSource = S;

    type FinalTarget = ();

    type AtInitial = ToUnit<S>;

    fn at_initial(&self) -> Self::AtInitial {
        ToUnit::mk()
    }

    type AtFinal = ToUnit<S>;

    fn at_final(&self) -> Self::AtFinal {
        ToUnit::mk()
    }
}

impl<S> PartialLens for ToUnit<S> {
    fn try_get(&self, _v: S) -> Result<(), S> {
        Ok(())
    }

    fn set(&self, v: S, _x: ()) -> S {
        v
    }

    fn exchange(&self, v: S, _x: ()) -> (Option<()>, S) {
        (Some(()), v)
    }

    fn modify<F: FnOnce(()) -> ()>(&self, v: S, _f: F) -> S {
        v
    }

    fn modify_with<F, X>(&self, v: S, f: F) -> (S, Option<X>)
        where F: FnOnce(()) -> ((), X)
    {
        (v, Some(f(()).1))
    }
}

impl<S> Lens for ToUnit<S> {
    fn get(&self, _v: S) -> () {
        ()
    }
}
