use std::rc::Rc;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::cmp::Ordering;
use std::fmt::{Formatter, Display, Debug, Error, Pointer};
use std::hash::{Hasher, Hash};

/// A smart container for objects in recursive data structures
#[derive(Default)]
pub struct SCell<T: ?Sized>(Rc<UnsafeCell<T>>);

/// A reference wrapper that lets rust make the same guarentees regardless of internal type
pub struct Ref<'a, T: 'a + ?Sized>(&'a T);

/// A mutable reference wrapper that lets rust make the same guarentees regardless of internal type
pub struct RefMut<'a, T: 'a + ?Sized>(&'a mut T);

impl<T> SCell<T> {
    fn new(t: T) -> Self {
        SCell(Rc::new(UnsafeCell::new(t)))
    }
}

impl<T: ?Sized> SCell<T> {
    fn borrow(&self) -> Ref<T> {
        Ref(unsafe{&*self.0.get() as &T})
    }

    fn borrow_mut(&self) -> RefMut<T> {
        RefMut(unsafe{&mut *self.0.get() as &mut T})
    }
}

impl<T: ?Sized> Clone for SCell<T> {
    fn clone(&self) -> Self {
        SCell(self.0.clone())
    }
}

impl<T: ?Sized> PartialEq for SCell<T>
    where T: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        *self.borrow() == *other.borrow()
    }

    fn ne(&self, other: &Self) -> bool {
        *self.borrow() != *other.borrow()
    }
}

impl<T: ?Sized> Eq for SCell<T> where T: Eq {}

impl<T: ?Sized> PartialOrd for SCell<T>
    where T: PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.borrow().partial_cmp(&*other.borrow())
    }

    fn lt(&self, other: &Self) -> bool {
        *self.borrow() < *other.borrow()
    }

    fn le(&self, other: &Self) -> bool {
        *self.borrow() <= *other.borrow()
    }

    fn gt(&self, other: &Self) -> bool {
        *self.borrow() > *other.borrow()
    }

    fn ge(&self, other: &Self) -> bool {
        *self.borrow() >= *other.borrow()
    }
}

impl<T: ?Sized> Ord for SCell<T>
    where T: Ord
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.borrow().cmp(&*other.borrow())
    }
}

impl<T: ?Sized> Hash for SCell<T>
    where T: Hash
{
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        self.borrow().hash(state);
    }
}

impl<T: ?Sized> Display for SCell<T>
    where T: Display
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.borrow().fmt(f)
    }
}

impl<T: ?Sized> Debug for SCell<T>
    where T: Debug
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.borrow().fmt(f)
    }
}

impl<T: ?Sized> Pointer for SCell<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

impl<T> From<T> for SCell<T> {
    fn from(t: T) -> Self {
        SCell::new(t)
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
