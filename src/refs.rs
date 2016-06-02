use std::marker::PhantomData;
use std::ops::{DerefMut, IndexMut};
use super::{Lens, Lenticuloid};

///
pub struct ToBorrowed<B: ToOwned> {
  phantom_bo: PhantomData<Fn(B) -> B::Owned>,
  phantom_ob: PhantomData<Fn(B::Owned) -> B>,
}
