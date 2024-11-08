use std::cell::UnsafeCell;
use std::fmt::{Debug, Formatter};

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

    pub fn get_or_init<F>(&self, init: F) -> &T where F: FnOnce() -> T {
        if self.get().is_none() {
            unsafe {
                *self.unsafe_cell.get() = Some(init());
            }
        }
        self.get().unwrap()
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

impl<T> Default for DeferredBox<T> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<T> Send for DeferredBox<T> where T: Send { }
unsafe impl<T> Sync for DeferredBox<T> where T: Sync { }

impl<T> Debug for DeferredBox<T> where T: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeferredBox {{ {:?} }})", self.get())
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct DeferredBoxSetError();

impl Debug for DeferredBoxSetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "value has been set, trying to set it twice")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deferred_box_for_number() {
        let deferred_box = DeferredBox::new();
        assert_eq!(deferred_box.get(), None);
        assert_eq!(deferred_box.set(1), Ok(()));
        assert_eq!(deferred_box.get(), Some(&1));
        assert_eq!(deferred_box.set(2), Err(DeferredBoxSetError()));
        assert_eq!(deferred_box.get(), Some(&1));
    }

    #[test]
    fn test_deferred_box_for_string() {
        let deferred_box = DeferredBox::new();
        assert_eq!(deferred_box.get(), None);
        assert_eq!(deferred_box.set("hello".to_string()), Ok(()));
        assert_eq!(deferred_box.get(), Some(&"hello".to_string()));
        assert_eq!(deferred_box.set("world".to_string()), Err(DeferredBoxSetError()));
        assert_eq!(deferred_box.get(), Some(&"hello".to_string()));
    }

    #[test]
    fn test_get_or_init() {
        let deferred_box = DeferredBox::new();
        assert_eq!(deferred_box.get(), None);
        assert_eq!(deferred_box.get_or_init(|| 1), &1);
        assert_eq!(deferred_box.get(), Some(&1));
        assert_eq!(deferred_box.get_or_init(|| 2), &1);
        assert_eq!(deferred_box.get(), Some(&1));
    }

    #[test]
    fn test_debug_message() {
        let deferred_box: DeferredBox<i32> = DeferredBox::new();
        deferred_box.set(50).unwrap();
        assert_eq!(&format!("{:?}", deferred_box), "DeferredBox { Some(50) })");
    }
}
