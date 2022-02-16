use crate::patterns::impl_newtype_generic;

impl_newtype_generic!(T: [Copy], core::cell::Cell<T>, core::cell::Cell::new);
impl_newtype_generic!(T: [], core::cell::RefCell<T>, core::cell::RefCell::new);
// Cannot impl for UnsafeCell because it is not Clone.
