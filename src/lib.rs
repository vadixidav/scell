#[cfg(not(feature = "unchecked"))]
mod checked;
#[cfg(not(feature = "unchecked"))]
pub use checked::*;

#[cfg(feature = "unchecked")]
mod unchecked;
#[cfg(feature = "unchecked")]
pub use unchecked::*;

use std::cmp::Ordering;
use std::fmt::{Formatter, Display, Debug, Error, Pointer};
use std::hash::{Hasher, Hash};

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

impl<T> From<T> for SCell<T> {
    fn from(t: T) -> Self {
        SCell::new(t)
    }
}
