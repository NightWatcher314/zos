use core::cell::{RefCell, RefMut};

pub struct UPSafeCell<T> {
    value: RefCell<T>,
}

unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    pub unsafe fn new(value: T) -> Self {
        Self {
            value: RefCell::new(value),
        }
    }

    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.value.borrow_mut()
    }
}
