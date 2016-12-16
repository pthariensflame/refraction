//! These lenticuloids deal with the trivial and the impossible; that is, they
//! operate with values of types `()` and `Never`.

use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
use super::{Lens, Lenticuloid, PartialLens, Prism};

#[cfg(feature = "nightly")]
/// A `Lens` to "extract" anything from `!`.
pub struct FromNever<A, B> {
    phantom_va: PhantomData<Fn(!) -> A>,
    phantom_bv: PhantomData<Fn(B) -> !>,
}

#[cfg(feature = "nightly")]
impl<A, B> FromNever<A, B> {
    pub fn mk() -> Self {
        FromNever { phantom_va: PhantomData,
                    phantom_bv: PhantomData, }
    }
}

#[cfg(feature = "nightly")]
impl<A, B> Debug for FromNever<A, B> {
    fn fmt(&self, fm: &mut Formatter) -> fmt::Result {
        fm.debug_struct("FromNever")
          .field("phantom_va", &self.phantom_va)
          .field("phantom_bv", &self.phantom_bv)
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
}

#[cfg(feature = "nightly")]
impl<A, B> PartialLens for FromNever<A, B> {
    fn try_get(&self, v: !) -> Result<A, !> {
        v
    }

    fn set(&self, v: !, _x: B) -> ! {
        v
    }

    fn modify<F: FnOnce(A) -> B>(&self, v: !, _f: F) -> ! {
        v
    }

    fn try_modify<F>(&self, v: !, f: F) -> Self::FinalSource
        where F: FnOnce(Result<A, !>) -> B
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
    phantom_vs: PhantomData<Fn(!) -> S>,
    phantom_sv: PhantomData<Fn(S) -> !>,
}

#[cfg(feature = "nightly")]
impl<S> ToNever<S> {
    pub fn mk() -> Self {
        ToNever { phantom_vs: PhantomData,
                  phantom_sv: PhantomData, }
    }
}

#[cfg(feature = "nightly")]
impl<S> Debug for ToNever<S> {
    fn fmt(&self, fm: &mut Formatter) -> fmt::Result {
        fm.debug_struct("ToNever")
          .field("phantom_vs", &self.phantom_vs)
          .field("phantom_sv", &self.phantom_sv)
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
}

#[cfg(feature = "nightly")]
impl<S> PartialLens for ToNever<S> {
    fn try_get(&self, v: S) -> Result<!, S> {
        Err(v)
    }

    fn set(&self, _v: S, x: !) -> S {
        x
    }

    fn modify<F>(&self, v: S, _f: F) -> S
        where F: FnOnce(!) -> !
    {
        v
    }

    fn try_modify<F>(&self, v: S, f: F) -> S
        where F: FnOnce(Result<!, S>) -> !
    {
        v
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
    pub fn mk() -> Self {
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
}

impl<A, B> PartialLens for FromUnit<A, B> {
    fn try_get(&self, v: ()) -> Result<A, ()> {
        Err(v)
    }

    fn set(&self, v: (), _x: B) -> () {
        v
    }

    fn modify<F>(&self, v: (), _f: F) -> ()
        where F: FnOnce(A) -> B
    {
        v
    }

    fn try_modify<F>(&self, v: (), f: F) -> ()
        where F: FnOnce(Result<A, ()>) -> B
    {
        v
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
    pub fn mk() -> Self {
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
}

impl<S> PartialLens for ToUnit<S> {
    fn try_get(&self, _v: S) -> Result<(), S> {
        Ok(())
    }

    fn set(&self, v: S, _x: ()) -> S {
        v
    }

    fn modify<F: FnOnce(()) -> ()>(&self, v: S, _f: F) -> S {
        v
    }

    fn try_modify<F>(&self, v: S, f: F) -> S
        where F: FnOnce(Result<(), S>) -> ()
    {
        v
    }
}

impl<S> Lens for ToUnit<S> {
    fn get(&self, _v: S) -> () {
        ()
    }
}
