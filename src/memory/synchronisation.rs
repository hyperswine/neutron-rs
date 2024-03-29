use core::cell::UnsafeCell;

// -------------------
// GENERIC MUTEX
// -------------------

pub trait Mutex {
    type Data;

    fn lock<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R;
}

pub trait ReadWriteEx {
    type Data;
    fn write<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R;
    fn read<R>(&self, f: impl FnOnce(&Self::Data) -> R) -> R;
}

// -------------------
// IRQ LOCK
// -------------------

pub struct IRQSafeNullLock<T>
where
    T: ?Sized,
{
    data: UnsafeCell<T>,
}

pub struct InitStateLock<T>
where
    T: ?Sized,
{
    data: UnsafeCell<T>,
}

unsafe impl<T> Send for IRQSafeNullLock<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for IRQSafeNullLock<T> where T: ?Sized + Send {}

impl<T> IRQSafeNullLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }
}

unsafe impl<T> Send for InitStateLock<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for InitStateLock<T> where T: ?Sized + Send {}

impl<T> InitStateLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }
}

use crate::exception;

impl<T> Mutex for IRQSafeNullLock<T> {
    type Data = T;

    fn lock<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
        let data = unsafe { &mut *self.data.get() };
        exception::exec_with_irq_masked(|| f(data))
    }
}

impl<T> ReadWriteEx for InitStateLock<T> {
    type Data = T;

    fn write<R>(&self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
        let data = unsafe { &mut *self.data.get() };

        f(data)
    }

    fn read<R>(&self, f: impl FnOnce(&Self::Data) -> R) -> R {
        let data = unsafe { &*self.data.get() };

        f(data)
    }
}
