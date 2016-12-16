//! These lenticuloids handle lossless conversions via the standard library
//! traits `Into`, `AsRef` and `AsMut`.

use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
use super::{Iso, Lens, Lenticuloid, PartialLens, Prism};

/// An isomorphism family that handles lossless conversions by owned value.
pub struct Conv<S, A, T = S, B = A> {
    phantom_sa: PhantomData<Fn(S) -> A>,
    phantom_bt: PhantomData<Fn(B) -> T>,
}

impl<S, A, T, B> Conv<S, A, T, B>
    where S: Into<A>,
          B: Into<T>
{
    #[inline]
    pub fn mk() -> Self {
        Conv { phantom_sa: PhantomData,
               phantom_bt: PhantomData, }
    }
}

impl<S, A, T, B> Debug for Conv<S, A, T, B>
    where S: Into<A>,
          B: Into<T>
{
    fn fmt(&self, fm: &mut Formatter) -> fmt::Result {
        fm.debug_struct("Conv")
          .field("phantom_sa", &self.phantom_sa)
          .field("phantom_bt", &self.phantom_bt)
          .finish()
    }
}

impl<S, A, T, B> Clone for Conv<S, A, T, B>
    where S: Into<A>,
          B: Into<T>
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        *self = *source;
    }
}

impl<S, A, T, B> Copy for Conv<S, A, T, B>
    where S: Into<A>,
          B: Into<T>
{
}

impl<S, A, T, B> Default for Conv<S, A, T, B>
    where S: Into<A>,
          B: Into<T>
{
    #[inline]
    fn default() -> Self {
        Self::mk()
    }
}

impl<S, A, T, B> Lenticuloid for Conv<S, A, T, B>
    where S: Into<A>,
          B: Into<T>
{
    type InitialSource = S;

    type InitialTarget = A;

    type FinalSource = T;

    type FinalTarget = B;
}

impl<S, A, T, B> PartialLens for Conv<S, A, T, B>
    where S: Into<A>,
          B: Into<T>
{
    #[inline]
    fn try_get(&self, v: Self::InitialSource) -> Result<Self::InitialTarget, Self::FinalSource> {
        Ok(v.into())
    }

    #[inline]
    fn set(&self, _v: Self::InitialSource, x: Self::FinalTarget) -> Self::FinalSource {
        x.into()
    }

    #[inline]
    fn modify<F>(&self, v: Self::InitialSource, f: F) -> Self::FinalSource
        where F: FnOnce(Self::InitialTarget) -> Self::FinalTarget
    {
        f(v.into()).into()
    }
}

impl<S, A, T, B> Lens for Conv<S, A, T, B>
    where S: Into<A>,
          B: Into<T>
{
    #[inline]
    fn get(&self, v: Self::InitialSource) -> Self::InitialTarget {
        v.into()
    }
}

impl<S, A, T, B> Prism for Conv<S, A, T, B>
    where S: Into<A>,
          B: Into<T>
{
    #[inline]
    fn inject(&self, v: Self::FinalTarget) -> Self::FinalSource {
        v.into()
    }
}

impl<S, A, T, B> Iso for Conv<S, A, T, B>
    where S: Into<A>,
          B: Into<T>
{
}

/// An isomorphism family that handles lossless conversions by shared reference.
pub struct ConvRef<'a, S: ?Sized + 'a, A: ?Sized + 'a, T: ?Sized + 'a, B: ?Sized + 'a> {
    phantom_sa: PhantomData<Fn(&'a S) -> &'a A>,
    phantom_bt: PhantomData<Fn(&'a B) -> &'a T>,
}

impl<'a, S: ?Sized, A: ?Sized, T: ?Sized, B: ?Sized> ConvRef<'a, S, A, T, B>
    where S: AsRef<A> + 'a,
          A: 'a,
          T: 'a,
          B: AsRef<T> + 'a
{
    #[inline]
    pub fn mk() -> Self {
        ConvRef { phantom_sa: PhantomData,
                  phantom_bt: PhantomData, }
    }
}

impl<'a, S: ?Sized, A: ?Sized, T: ?Sized, B: ?Sized> Debug for ConvRef<'a, S, A, T, B>
    where S: AsRef<A> + 'a,
          A: 'a,
          T: 'a,
          B: AsRef<T> + 'a
{
    fn fmt(&self, fm: &mut Formatter) -> fmt::Result {
        fm.debug_struct("ConvRef")
          .field("phantom_sa", &self.phantom_sa)
          .field("phantom_bt", &self.phantom_bt)
          .finish()
    }
}

impl<'a, S: ?Sized, A: ?Sized, T: ?Sized, B: ?Sized> Clone for ConvRef<'a, S, A, T, B>
    where S: AsRef<A> + 'a,
          A: 'a,
          T: 'a,
          B: AsRef<T> + 'a
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        *self = *source;
    }
}

impl<'a, S: ?Sized, A: ?Sized, T: ?Sized, B: ?Sized> Copy for ConvRef<'a, S, A, T, B>
    where S: AsRef<A> + 'a,
          A: 'a,
          T: 'a,
          B: AsRef<T> + 'a
{
}

impl<'a, S: ?Sized, A: ?Sized, T: ?Sized, B: ?Sized> Default for ConvRef<'a, S, A, T, B>
    where S: AsRef<A> + 'a,
          A: 'a,
          T: 'a,
          B: AsRef<T> + 'a
{
    #[inline]
    fn default() -> Self {
        Self::mk()
    }
}

impl<'a, S: ?Sized, A: ?Sized, T: ?Sized, B: ?Sized> Lenticuloid for ConvRef<'a, S, A, T, B>
    where S: AsRef<A> + 'a,
          A: 'a,
          T: 'a,
          B: AsRef<T> + 'a
{
    type InitialSource = &'a S;

    type InitialTarget = &'a A;

    type FinalSource = &'a T;

    type FinalTarget = &'a B;
}

impl<'a, S: ?Sized, A: ?Sized, T: ?Sized, B: ?Sized> PartialLens for ConvRef<'a, S, A, T, B>
    where S: AsRef<A> + 'a,
          A: 'a,
          T: 'a,
          B: AsRef<T> + 'a
{
    #[inline]
    fn try_get(&self, v: Self::InitialSource) -> Result<Self::InitialTarget, Self::FinalSource> {
        Ok(v.as_ref())
    }

    #[inline]
    fn set(&self, _v: Self::InitialSource, x: Self::FinalTarget) -> Self::FinalSource {
        x.as_ref()
    }

    #[inline]
    fn modify<F>(&self, v: Self::InitialSource, f: F) -> Self::FinalSource
        where F: FnOnce(Self::InitialTarget) -> Self::FinalTarget
    {
        f(v.as_ref()).as_ref()
    }
}

impl<'a, S: ?Sized, A: ?Sized, T: ?Sized, B: ?Sized> Lens for ConvRef<'a, S, A, T, B>
    where S: AsRef<A> + 'a,
          A: 'a,
          T: 'a,
          B: AsRef<T> + 'a
{
    #[inline]
    fn get(&self, v: Self::InitialSource) -> Self::InitialTarget {
        v.as_ref()
    }
}

impl<'a, S: ?Sized, A: ?Sized, T: ?Sized, B: ?Sized> Prism for ConvRef<'a, S, A, T, B>
    where S: AsRef<A> + 'a,
          A: 'a,
          T: 'a,
          B: AsRef<T> + 'a
{
    #[inline]
    fn inject(&self, v: Self::FinalTarget) -> Self::FinalSource {
        v.as_ref()
    }
}

impl<'a, S: ?Sized, A: ?Sized, T: ?Sized, B: ?Sized> Iso for ConvRef<'a, S, A, T, B>
    where S: AsRef<A> + 'a,
          A: 'a,
          T: 'a,
          B: AsRef<T> + 'a
{
}

/// An isomorphism family that handles lossless conversions by mutable
/// reference.
pub struct ConvMut<'a, S: ?Sized + 'a, A: ?Sized + 'a, T: ?Sized + 'a, B: ?Sized + 'a> {
    phantom_sa: PhantomData<Fn(&'a mut S) -> &'a mut A>,
    phantom_bt: PhantomData<Fn(&'a mut B) -> &'a mut T>,
}

impl<'a, S: ?Sized, A: ?Sized, T: ?Sized, B: ?Sized> ConvMut<'a, S, A, T, B>
    where S: AsMut<A> + 'a,
          A: 'a,
          T: 'a,
          B: AsMut<T> + 'a
{
    #[inline]
    pub fn mk() -> Self {
        ConvMut { phantom_sa: PhantomData,
                  phantom_bt: PhantomData, }
    }
}

impl<'a, S: ?Sized, A: ?Sized, T: ?Sized, B: ?Sized> Debug for ConvMut<'a, S, A, T, B>
    where S: AsMut<A> + 'a,
          A: 'a,
          T: 'a,
          B: AsMut<T> + 'a
{
    fn fmt(&self, fm: &mut Formatter) -> fmt::Result {
        fm.debug_struct("ConvMut")
          .field("phantom_sa", &self.phantom_sa)
          .field("phantom_bt", &self.phantom_bt)
          .finish()
    }
}

impl<'a, S: ?Sized, A: ?Sized, T: ?Sized, B: ?Sized> Clone for ConvMut<'a, S, A, T, B>
    where S: AsMut<A> + 'a,
          A: 'a,
          T: 'a,
          B: AsMut<T> + 'a
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        *self = *source;
    }
}

impl<'a, S: ?Sized, A: ?Sized, T: ?Sized, B: ?Sized> Copy for ConvMut<'a, S, A, T, B>
    where S: AsMut<A> + 'a,
          A: 'a,
          T: 'a,
          B: AsMut<T> + 'a
{
}

impl<'a, S: ?Sized, A: ?Sized, T: ?Sized, B: ?Sized> Default for ConvMut<'a, S, A, T, B>
    where S: AsMut<A> + 'a,
          A: 'a,
          T: 'a,
          B: AsMut<T> + 'a
{
    #[inline]
    fn default() -> Self {
        Self::mk()
    }
}

impl<'a, S: ?Sized, A: ?Sized, T: ?Sized, B: ?Sized> Lenticuloid for ConvMut<'a, S, A, T, B>
    where S: AsMut<A> + 'a,
          A: 'a,
          T: 'a,
          B: AsMut<T> + 'a
{
    type InitialSource = &'a mut S;

    type InitialTarget = &'a mut A;

    type FinalSource = &'a mut T;

    type FinalTarget = &'a mut B;
}

impl<'a, S: ?Sized, A: ?Sized, T: ?Sized, B: ?Sized> PartialLens for ConvMut<'a, S, A, T, B>
    where S: AsMut<A> + 'a,
          A: 'a,
          T: 'a,
          B: AsMut<T> + 'a
{
    #[inline]
    fn try_get(&self, v: Self::InitialSource) -> Result<Self::InitialTarget, Self::FinalSource> {
        Ok(v.as_mut())
    }

    #[inline]
    fn set(&self, _v: Self::InitialSource, x: Self::FinalTarget) -> Self::FinalSource {
        x.as_mut()
    }

    #[inline]
    fn modify<F>(&self, v: Self::InitialSource, f: F) -> Self::FinalSource
        where F: FnOnce(Self::InitialTarget) -> Self::FinalTarget
    {
        f(v.as_mut()).as_mut()
    }
}

impl<'a, S: ?Sized, A: ?Sized, T: ?Sized, B: ?Sized> Lens for ConvMut<'a, S, A, T, B>
    where S: AsMut<A> + 'a,
          A: 'a,
          T: 'a,
          B: AsMut<T> + 'a
{
    #[inline]
    fn get(&self, v: Self::InitialSource) -> Self::InitialTarget {
        v.as_mut()
    }
}

impl<'a, S: ?Sized, A: ?Sized, T: ?Sized, B: ?Sized> Prism for ConvMut<'a, S, A, T, B>
    where S: AsMut<A> + 'a,
          A: 'a,
          T: 'a,
          B: AsMut<T> + 'a
{
    #[inline]
    fn inject(&self, v: Self::FinalTarget) -> Self::FinalSource {
        v.as_mut()
    }
}

impl<'a, S: ?Sized, A: ?Sized, T: ?Sized, B: ?Sized> Iso for ConvMut<'a, S, A, T, B>
    where S: AsMut<A> + 'a,
          A: 'a,
          T: 'a,
          B: AsMut<T> + 'a
{
}
