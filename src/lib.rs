use std::cell::UnsafeCell;
use std::fmt::Debug;

pub struct DeferredBox<T> {
    unsafe_cell: UnsafeCell<Option<T>>,
}

impl <T> DeferredBox<T> {
    pub fn new() -> Self {
        Self {
            unsafe_cell: UnsafeCell::new(None),
        }
    }

    pub fn get(&self) -> Option<&T> {
        (unsafe { &*self.unsafe_cell.get() }).as_ref()
    }

    pub unsafe fn get_mut(&self) -> Option<&mut T> {
        { &mut *self.unsafe_cell.get() }.as_mut()
    }

    pub unsafe fn alter(&self, value: T) {
        *self.unsafe_cell.get() = Some(value);
    }

    pub fn set(&self, value: T) -> Result<(), DeferredBoxSetError> {
        if self.get().is_some() {
            return Err(DeferredBoxSetError());
        }
        unsafe {
            *self.unsafe_cell.get() = Some(value);
        }
        Ok(())
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct DeferredBoxSetError();

impl Debug for DeferredBoxSetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value has been set, trying to set it twice")
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_deferred_box_for_number() {
        let deferred_box = super::DeferredBox::new();
        assert_eq!(deferred_box.get(), None);
        assert_eq!(deferred_box.set(1), Ok(()));
        assert_eq!(deferred_box.get(), Some(&1));
        assert_eq!(deferred_box.set(2), Err(super::DeferredBoxSetError()));
        assert_eq!(deferred_box.get(), Some(&1));
    }

    #[test]
    fn test_deferred_box_for_string() {
        let deferred_box = super::DeferredBox::new();
        assert_eq!(deferred_box.get(), None);
        assert_eq!(deferred_box.set("hello".to_string()), Ok(()));
        assert_eq!(deferred_box.get(), Some(&"hello".to_string()));
        assert_eq!(deferred_box.set("world".to_string()), Err(super::DeferredBoxSetError()));
        assert_eq!(deferred_box.get(), Some(&"hello".to_string()));
    }
}