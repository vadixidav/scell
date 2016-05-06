#[cfg(not(feature = "unchecked"))]
mod checked;
#[cfg(not(feature = "unchecked"))]
pub use checked::*;

#[cfg(feature = "unchecked")]
mod unchecked;
#[cfg(feature = "unchecked")]
pub use unchecked::*;
