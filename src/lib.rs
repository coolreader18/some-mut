#![no_std]

use core::borrow::Borrow;
use core::fmt;
use core::ops::{Deref, DerefMut};

pub struct SomeMut<'a, T>(
    /// INVARIANT: must always be Option::Some
    &'a mut Option<T>,
);

mod sealed {
    pub trait Sealed {}
}

pub trait OptionExt<T>: sealed::Sealed {
    fn some_mut(&mut self) -> Option<SomeMut<'_, T>>;
}

impl<T> sealed::Sealed for Option<T> {}
impl<T> OptionExt<T> for Option<T> {
    fn some_mut(&mut self) -> Option<SomeMut<'_, T>> {
        self.is_some().then_some(SomeMut(self))
    }
}

impl<'a, T> SomeMut<'a, T> {
    pub fn take(self) -> T {
        // SAFETY: safety invariant on SomeMut
        unsafe { self.0.take().unwrap_unchecked() }
    }

    pub fn into_mut(self) -> &'a mut T {
        // SAFETY: safety invariant on SomeMut
        unsafe { self.0.as_mut().unwrap_unchecked() }
    }

    pub fn into_option_mut(self) -> &'a mut Option<T> {
        self.0
    }
}

impl<T> Deref for SomeMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: safety invariant on SomeMut
        unsafe { self.0.as_ref().unwrap_unchecked() }
    }
}

impl<T> DerefMut for SomeMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: safety invariant on SomeMut
        unsafe { self.0.as_mut().unwrap_unchecked() }
    }
}

impl<T: fmt::Debug> fmt::Debug for SomeMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: fmt::Display> fmt::Display for SomeMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<T> Borrow<T> for SomeMut<'_, T> {
    fn borrow(&self) -> &T {
        self
    }
}

impl<T> AsRef<T> for SomeMut<'_, T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T> AsMut<T> for SomeMut<'_, T> {
    fn as_mut(&mut self) -> &mut T {
        self
    }
}

impl<T: PartialEq> PartialEq<T> for SomeMut<'_, T> {
    fn eq(&self, other: &T) -> bool {
        T::eq(self, other)
    }
}

impl<T: PartialOrd> PartialOrd<T> for SomeMut<'_, T> {
    fn partial_cmp(&self, other: &T) -> Option<core::cmp::Ordering> {
        T::partial_cmp(self, other)
    }
}
