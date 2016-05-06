# scell
A smart cell for Rust that allows feature gating for disabling runtime-checked borrows and provides a simple interface

The intention of this crate is to allow the use of a `Rc<RefCell<T>>` throughout code where, after enough testing
has been done, the `unchecked` feature can be enabled, which will convert SCell into a `Rc<UnsafeCell<T>>`.

SCell provides all of the things that the combination of `Rc<RefCell<T>>` normally allow and some more, such as
implementations for `PartialOrd` and `Ord`.

If you plan to do significant testing in `debug` mode, add the `unchecked` feature to this crate in `release` mode.
Otherwise, it might be best to enable optimizations in `debug` so you can test in `debug` or to create a new
profile for testing of optimized binaries that still do the runtime checking that RefCell provides. Once you have
performed your testing, use a compile mode with the `unchecked` feature enabled for this crate and it will remove
the overhead from `RefCell`, but not from `Rc` since it still needs to know when to `drop()` the value.

Alternatively, feel free to use this crate for normal use in graphs, meshes, and other recurrent data structures
with lots of interconnectivity where the borrow checker simply can't help. Later, if your code works fine and you
need the performance back from `RefCell`, just use the `unchecked` feature and your code will be good to go.
