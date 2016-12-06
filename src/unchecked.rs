use std::rc::Rc;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::fmt::{Formatter, Debug, Error, Pointer};
use std::cmp::Ordering;

/// A smart container for objects in recursive data structures
///
/// This container contains Rc and therefore `clone()` will create a new reference to the same instance.
#[derive(Default)]
pub struct SCell<T: ?Sized>(Rc<UnsafeCell<T>>);

/// A reference wrapper that lets rust make the same guarantees regardless of internal type
pub struct Ref<'a, T: 'a + ?Sized>(&'a T);

/// A mutable reference wrapper that lets rust make the same guarantees regardless of internal type
pub struct RefMut<'a, T: 'a + ?Sized>(&'a mut T);

impl<T> SCell<T> {
    #[inline]
    pub fn new(t: T) -> Self {
        SCell(Rc::new(UnsafeCell::new(t)))
    }
}

impl<T: ?Sized> SCell<T> {
    #[inline]
    pub fn borrow(&self) -> Ref<T> {
        Ref(unsafe{&*self.0.get() as &T})
    }

    #[inline]
    pub fn borrow_mut(&self) -> RefMut<T> {
        RefMut(unsafe{&mut *self.0.get() as &mut T})
    }
}

impl<T: ?Sized> Clone for SCell<T> {
    #[inline]
    fn clone(&self) -> Self {
        SCell(self.0.clone())
    }
}

impl<T: ?Sized> Pointer for SCell<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

impl<'a, T: 'a + ?Sized> Deref for Ref<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &*self.0
    }
}

impl<T: ?Sized> PartialEq for SCell<T>
where T: PartialEq
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        *self.borrow() == *other.borrow()
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        *self.borrow() != *other.borrow()
    }
}

impl<T: ?Sized> Eq for SCell<T> where T: Eq {}

impl<T: ?Sized> PartialOrd for SCell<T>
where T: PartialOrd
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.borrow().partial_cmp(&*other.borrow())
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        *self.borrow() < *other.borrow()
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        *self.borrow() <= *other.borrow()
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        *self.borrow() > *other.borrow()
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        *self.borrow() >= *other.borrow()
    }
}

impl<T: ?Sized> Ord for SCell<T>
where T: Ord
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.borrow().cmp(&*other.borrow())
    }
}

impl<'a, T: 'a + ?Sized> Debug for Ref<'a, T>
    where T: Debug
{
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        (*self.0).fmt(f)
    }
}

impl<'a, T: 'a + ?Sized> Deref for RefMut<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        &*self.0
    }
}

impl<'a, T: 'a + ?Sized> DerefMut for RefMut<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.0
    }
}

impl<'a, T: 'a + ?Sized> Debug for RefMut<'a, T>
    where T: Debug
{
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        (*self.0).fmt(f)
    }
}
