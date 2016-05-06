use std::rc::Rc;
use std::cell;
use std::ops::{Deref, DerefMut};
use std::fmt::{Formatter, Display, Debug, Error, Pointer};

/// A smart container for objects in recursive data structures
///
/// This container contains Rc and therefore `clone()` will create a new reference to the same instance.
#[derive(Default)]
pub struct SCell<T: ?Sized>(Rc<cell::RefCell<T>>);

/// A reference wrapper that lets rust make the same guarentees regardless of internal type
pub struct Ref<'a, T: 'a + ?Sized>(cell::Ref<'a, T>);

/// A mutable reference wrapper that lets rust make the same guarentees regardless of internal type
pub struct RefMut<'a, T: 'a + ?Sized>(cell::RefMut<'a, T>);

impl<T> SCell<T> {
    pub fn new(t: T) -> Self {
        SCell(Rc::new(cell::RefCell::new(t)))
    }
}

impl<T: ?Sized> SCell<T> {
    pub fn borrow(&self) -> Ref<T> {
        Ref(self.0.borrow())
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        RefMut(self.0.borrow_mut())
    }
}

impl<T: ?Sized> Clone for SCell<T> {
    fn clone(&self) -> Self {
        SCell(self.0.clone())
    }
}

impl<T: ?Sized> Pointer for SCell<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

impl<'a, T: 'a + ?Sized> Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.0
    }
}

impl<'a, T: 'a + ?Sized> Debug for Ref<'a, T>
    where T: Debug
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        (*self.0).fmt(f)
    }
}

impl<'a, T: 'a + ?Sized> Deref for RefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.0
    }
}

impl<'a, T: 'a + ?Sized> DerefMut for RefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.0
    }
}

impl<'a, T: 'a + ?Sized> Debug for RefMut<'a, T>
    where T: Debug
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        (*self.0).fmt(f)
    }
}
