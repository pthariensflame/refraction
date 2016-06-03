//! These lenticuloids deal with the trivial and the impossible; that is, they
//! operate with values of types `()` and `Void`.
use std::marker::PhantomData;
use std::fmt::{self, Debug, Formatter};
use void::{Void, unreachable};
use super::{Lens, Lenticuloid, PartialLens, Prism};

/// A `Lens` to "extract" anything from `Void`.
pub struct FromVoid<A, B> {
  phantom_va: PhantomData<Fn(Void) -> A>,
  phantom_bv: PhantomData<Fn(B) -> Void>,
}

impl<A, B> FromVoid<A, B> {
  pub fn mk() -> Self {
    FromVoid {
      phantom_va: PhantomData,
      phantom_bv: PhantomData,
    }
  }
}

impl<A, B> Debug for FromVoid<A, B> {
  fn fmt(&self, fm: &mut Formatter) -> fmt::Result {
    fm.debug_struct("FromVoid")
      .field("phantom_va", &self.phantom_va)
      .field("phantom_bv", &self.phantom_bv)
      .finish()
  }
}

impl<A, B> Clone for FromVoid<A, B> {
  fn clone(&self) -> Self { *self }

  fn clone_from(&mut self, source: &Self) { *self = *source; }
}

impl<A, B> Copy for FromVoid<A, B> {}

impl<A, B> Default for FromVoid<A, B> {
  fn default() -> Self { Self::mk() }
}

impl<A, B> Lenticuloid for FromVoid<A, B> {
  type InitialSource = Void;

  type InitialTarget = A;

  type FinalSource = Void;

  type FinalTarget = B;
}

impl<A, B> PartialLens for FromVoid<A, B> {
  fn try_get(&self, v: Void) -> Result<A, Void> { unreachable(v) }

  fn set(&self, v: Void, _x: B) -> Void { unreachable(v) }

  fn modify<F: FnOnce(A) -> B>(&self, v: Void, _f: F) -> Void { unreachable(v) }
}

impl<A, B> Lens for FromVoid<A, B> {
  fn get(&self, v: Void) -> A { unreachable(v) }
}

/// A `Prism` to "inject" `Void` into anything.
pub struct ToVoid<S> {
  phantom_vs: PhantomData<Fn(Void) -> S>,
  phantom_sv: PhantomData<Fn(S) -> Void>,
}

impl<S> ToVoid<S> {
  pub fn mk() -> Self {
    ToVoid {
      phantom_vs: PhantomData,
      phantom_sv: PhantomData,
    }
  }
}

impl<S> Debug for ToVoid<S> {
  fn fmt(&self, fm: &mut Formatter) -> fmt::Result {
    fm.debug_struct("ToVoid")
      .field("phantom_vs", &self.phantom_vs)
      .field("phantom_sv", &self.phantom_sv)
      .finish()
  }
}

impl<S> Clone for ToVoid<S> {
  fn clone(&self) -> Self { *self }

  fn clone_from(&mut self, source: &Self) { *self = *source; }
}

impl<S> Copy for ToVoid<S> {}

impl<S> Default for ToVoid<S> {
  fn default() -> Self { Self::mk() }
}

impl<S> Lenticuloid for ToVoid<S> {
  type InitialSource = S;

  type InitialTarget = Void;

  type FinalSource = S;

  type FinalTarget = Void;
}

impl<S> PartialLens for ToVoid<S> {
  fn try_get(&self, v: S) -> Result<Void, S> { Err(v) }

  fn set(&self, _v: S, x: Void) -> S { unreachable(x) }

  fn modify<F>(&self, v: S, _f: F) -> S
    where F: FnOnce(Void) -> Void {
    v
  }
}

impl<S> Prism for ToVoid<S> {
  fn inject(&self, v: Void) -> S { unreachable(v) }
}

/// A `Prism` to "inject" anything into `()`.
pub struct FromUnit<A, B> {
  phantom_ua: PhantomData<Fn(()) -> A>,
  phantom_bu: PhantomData<Fn(B) -> ()>,
}

impl<A, B> FromUnit<A, B> {
  pub fn mk() -> Self {
    FromUnit {
      phantom_ua: PhantomData,
      phantom_bu: PhantomData,
    }
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
  fn clone(&self) -> Self { *self }

  fn clone_from(&mut self, source: &Self) { *self = *source; }
}

impl<A, B> Copy for FromUnit<A, B> {}

impl<A, B> Default for FromUnit<A, B> {
  fn default() -> Self { Self::mk() }
}

impl<A, B> Lenticuloid for FromUnit<A, B> {
  type InitialSource = ();

  type InitialTarget = A;

  type FinalSource = ();

  type FinalTarget = B;
}

impl<A, B> PartialLens for FromUnit<A, B> {
  fn try_get(&self, v: ()) -> Result<A, ()> { Err(v) }

  fn set(&self, v: (), _x: B) -> () { v }

  fn modify<F>(&self, v: (), _f: F) -> ()
    where F: FnOnce(A) -> B {
    v
  }
}

impl<A, B> Prism for FromUnit<A, B> {
  fn inject(&self, _v: B) -> () { () }
}

/// A `Lens` to "extract" `()` from anything.
pub struct ToUnit<S> {
  phantom_su: PhantomData<Fn(S) -> ()>,
  phantom_us: PhantomData<Fn(()) -> S>,
}

impl<S> ToUnit<S> {
  pub fn mk() -> Self {
    ToUnit {
      phantom_su: PhantomData,
      phantom_us: PhantomData,
    }
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
  fn clone(&self) -> Self { *self }

  fn clone_from(&mut self, source: &Self) { *self = *source; }
}

impl<S> Copy for ToUnit<S> {}

impl<S> Default for ToUnit<S> {
  fn default() -> Self { Self::mk() }
}

impl<S> Lenticuloid for ToUnit<S> {
  type InitialSource = S;

  type InitialTarget = ();

  type FinalSource = S;

  type FinalTarget = ();
}

impl<S> PartialLens for ToUnit<S> {
  fn try_get(&self, _v: S) -> Result<(), S> { Ok(()) }

  fn set(&self, v: S, _x: ()) -> S { v }

  fn modify<F: FnOnce(()) -> ()>(&self, v: S, _f: F) -> S { v }
}

impl<S> Lens for ToUnit<S> {
  fn get(&self, _v: S) -> () { () }
}
