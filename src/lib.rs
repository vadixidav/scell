//! The intention of this crate is to allow the use of a `Rc<RefCell<T>>` throughout code where, after enough testing
//! has been done, the `unchecked` feature can be enabled, which will convert `SCell` into a `Rc<UnsafeCell<T>>`.
//!
//! `SCell` provides all of the things that the combination of `Rc<RefCell<T>>` normally allow and some more, such as
//! implementations for `PartialOrd` and `Ord`.
//!
//! If you plan to do significant testing in `debug` mode, add the `unchecked` feature to this crate in `release` mode.
//! Otherwise, it might be best to enable optimizations in `debug` so you can test in `debug` or to create a new
//! profile for testing of optimized binaries that still do the runtime checking that RefCell provides. Once you have
//! performed your testing, use a compile mode with the `unchecked` feature enabled for this crate and it will remove
//! the overhead from `RefCell`, but not from `Rc` since it still needs to know when to `drop()` the value.
//!
//! Alternatively, feel free to use this crate for normal use in graphs, meshes, and other recurrent data structures
//! with lots of interconnectivity where the borrow checker simply can't help. Later, if your code works fine and you
//! need the performance back from `RefCell`, just use the `unchecked` feature and your code will be good to go.

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

impl<T: ?Sized> Hash for SCell<T>
    where T: Hash
{
    #[inline]
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        self.borrow().hash(state);
    }
}

impl<T: ?Sized> Display for SCell<T>
    where T: Display
{
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.borrow().fmt(f)
    }
}

impl<T: ?Sized> Debug for SCell<T>
    where T: Debug
{
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.borrow().fmt(f)
    }
}

impl<T> From<T> for SCell<T> {
    #[inline]
    fn from(t: T) -> Self {
        SCell::new(t)
    }
}
