use super::{Compose, Identity, Invert, Lenticuloid};

#[cfg(not(feature = "nightly"))]
/// The identity lenticuloid (function form).
#[inline]
pub fn identity<S, T>() -> Identity<S, T> {
    Identity::mk()
}

#[cfg(feature = "nightly")]
/// The identity lenticuloid (constant function form).
#[inline]
pub const fn identity<S, T>() -> Identity<S, T> {
    Identity::mk()
}

/// Extension `trait` for lenticuloid composition in categorical order.
pub trait ComposeExt<Other>: Lenticuloid + Sized
    where Other: Lenticuloid<InitialTarget = Self::InitialSource, FinalTarget = Self::FinalSource>
{
    fn compose(self, other: Other) -> Compose<Self, Other>;
}

impl<This, Other> ComposeExt<Other> for This
    where This: Lenticuloid,
          Other: Lenticuloid<InitialTarget = This::InitialSource, FinalTarget = This::FinalSource>
{
    #[inline]
    fn compose(self, other: Other) -> Compose<Self, Other> {
        Compose::of(self, other)
    }
}

/// Compose all the provided lenticuloids in categorical order.
#[macro_export]
macro_rules! chain_compose {
    () => { $crate::Identity::mk() };
    ($l:expr) => { $l };
    ($lf:expr, $($ls:expr),+) => { $crate::Compose::of($lf, chain_compose!($($ls),+)) };
}

/// Extension `trait` for lenticuloid composition in intuitive order.
pub trait AndThenExt<Other: Lenticuloid>
    : Lenticuloid<InitialTarget = Other::InitialSource, FinalTarget = Other::FinalSource> + Sized
    where Other: Lenticuloid
{
    fn and_then(self, other: Other) -> Compose<Other, Self>;
}

impl<This, Other> AndThenExt<Other> for This
    where Other: Lenticuloid,
          This: Lenticuloid<InitialTarget = Other::InitialSource, FinalTarget = Other::FinalSource>
{
    #[inline]
    fn and_then(self, other: Other) -> Compose<Other, Self> {
        Compose::of(other, self)
    }
}

/// Compose all the provided lenticuloids in intuitive order.
#[macro_export]
macro_rules! chain_and_then {
    () => { $crate::Identity::mk() };
    ($l:expr) => { $l };
    ($lf:expr, $($ls:expr),+) => { $crate::Compose::of(chain_and_then!($($ls),+), $lf) };
}

/// Extension `trait` for lenticuloid inversion.
pub trait InvertExt: Lenticuloid + Sized {
    fn invert(self) -> Invert<Self>;
}

impl<This: Lenticuloid> InvertExt for This {
    #[inline]
    fn invert(self) -> Invert<Self> {
        Invert::of(self)
    }
}

/// Create a simple lens inline to address a specific (possibly nested) field
/// of a type.
#[macro_export]
macro_rules! field_lens {
    ($source:ty => $($field_name:tt).*: $target:ty) => {
        {
            #[derive(Copy,Clone,Debug,Default)]
            struct __FieldLens__;
            impl $crate::Lenticuloid for __FieldLens__ {
                type InitialSource = $source;
                type InitialTarget = $target;
                type FinalSource = $source;
                type FinalTarget = $target;
                type AtInitial = Self;
                #[inline]
                fn at_initial(&self) -> Self::AtInitial {
                    *self
                }
                type AtFinal = Self;
                #[inline]
                fn at_final(&self) -> Self::AtFinal {
                    *self
                }
            }
            impl $crate::PartialLens for __FieldLens__ {
                #[inline]
                fn try_get(&self, v: Self::InitialSource) ->
                    $crate::std::result::Result<Self::InitialTarget, Self::FinalSource>
                {
                    $crate::std::result::Result::Ok(v$(.$field_name)*)
                }
                #[inline]
                fn try_get_inject(&self, mut v: Self::InitialSource) ->
                    $crate::std::result::Result<(Self::InitialTarget,
                                                 $crate::Injector<Self::FinalTarget,
                                                 Self::FinalSource>), Self::FinalSource>
                {
                    // this is safe because we fully own `v` and can NoDrop-wrap it
                    let x = $crate::std::mem::replace(&mut v$(.$field_name)*, unsafe {
                        $crate::std::mem::uninitialized()
                    });
                    let v_no_drop = $crate::nodrop::NoDrop::new(v);
                    $crate::std::result::Result::Ok((
                        x,
                        $crate::util::once_to_mut(move |y| {
                            let mut v_final = v_no_drop.into_inner();
                            $crate::std::mem::forget($crate::std::mem::replace(
                                &mut v_final$(.$field_name)*,
                                y
                            ));
                            v_final
                        })
                    ))
                }
                #[inline]
                fn set(&self, mut v: Self::InitialSource, x: Self::FinalTarget) ->
                    Self::FinalSource
                {
                    v$(.$field_name)* = x;
                    v
                }
                #[inline]
                fn exchange(&self,
                            mut v: Self::InitialSource,
                            mut x: Self::FinalTarget) ->
                    ($crate::std::option::Option<Self::InitialTarget>,
                     Self::FinalSource)
                {
                    $crate::std::mem::swap(&mut v$(.$field_name)*, &mut x);
                    ($crate::std::option::Option::Some(x), v)
                }
                #[inline]
                fn modify<F>(&self, mut v: Self::InitialSource, f: F) -> Self::FinalSource
                    where F: FnOnce(Self::InitialTarget) -> Self::FinalTarget
                {
                    v$(.$field_name)* = f(v$(.$field_name)*);
                    v
                }
                #[inline]
                fn modify_with<F, X>(&self, mut v: Self::InitialSource, f: F) ->
                    (Self::FinalSource, $crate::std::option::Option<X>)
                    where F: FnOnce(Self::InitialTarget) -> (Self::FinalTarget, X)
                {
                    let (x, aux) = f(v$(.$field_name)*);
                    v$(.$field_name)* = x;
                    (v, $crate::std::option::Option::Some(aux))
                }
            }
            impl $crate::Lens for __FieldLens__ {
                #[inline]
                fn get(&self, v: Self::InitialSource) -> Self::InitialTarget
                {
                    v$(.$field_name)*
                }
            }
            __FieldLens__
        }
    }
}

#[cfg(test)]
mod test {
    use ::PartialLens;
    #[test]
    fn test_field_lens() {
        struct TestInner(String);
        struct TestOuter {
            pub test_field: TestInner,
        }
        let l = field_lens!(TestOuter => test_field.0: String);
        let v = TestOuter { test_field: TestInner("  hello\n".to_string()) };
        let w = l.modify(v, |x| x.trim().to_string());
        assert_eq!(w.test_field.0, "hello")
    }
}
