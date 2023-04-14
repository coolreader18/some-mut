//! A utility library that mainly lets you access a `Some` and then `take()` it infallibly.
//!
//! Useful, for example, in a `Future` implementation, when you might re-enter into
//! a function multiple times and so can't `take()` until a sub-future is `Ready`:
//!
//! ```
//! # use std::task::{ready, Context, Poll};
//! # use std::pin::Pin;
//! use some_mut::OptionExt;
//! # type Error = ();
//! # struct X { buffered_item: Option<u32>, sink: Sink }
//! # impl X {
//! # fn project(&mut self) -> &mut Self { self }
//! // for a theoretical `StreamExt::forward()`/`SinkExt::send_all()` implementation:
//! fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
//!     let me = self.project();
//!     if let Some(buffered_item) = me.buffered_item.some_mut() {
//!         ready!(me.sink.poll_ready(cx))?;
//!         me.sink.start_send(buffered_item.take())?;
//!     }
//!     // ...
//! #   Poll::Ready(Ok(()))
//! }
//! # }
//! # struct Sink;
//! # impl Sink {
//! #     fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> { Poll::Ready(Ok(())) }
//! #     fn start_send(&mut self, _item: u32) -> Result<(), Error> { Ok(()) }
//! # }
//! ```

#![no_std]
#![deny(missing_docs)]

use core::borrow::Borrow;
use core::fmt;
use core::ops::{Deref, DerefMut};

/// A mutable reference to an `Option` that is guaranteed to always be `Some`.
pub struct SomeMut<'a, T>(
    /// INVARIANT: must always be Option::Some
    &'a mut Option<T>,
);

mod sealed {
    pub trait Sealed {}
}

/// An extension trait that allows one to obtain a [`SomeMut`].
pub trait OptionExt<T>: sealed::Sealed {
    /// Obtain a `SomeMut<T>` from a `&mut Option<T>`.
    ///
    /// See also [`Option::as_mut()`] if [`take()`][Option::take]ing isn't required.
    fn some_mut(&mut self) -> Option<SomeMut<'_, T>>;
}

impl<T> sealed::Sealed for Option<T> {}
impl<T> OptionExt<T> for Option<T> {
    fn some_mut(&mut self) -> Option<SomeMut<'_, T>> {
        match self {
            x @ Some(_) => Some(SomeMut(x)),
            None => None,
        }
    }
}

impl<'a, T> SomeMut<'a, T> {
    /// Take the value from this `SomeMut`, leaving a `None` in the original option.
    ///
    /// # Examples
    ///
    /// ```
    /// use some_mut::OptionExt;
    ///
    /// let mut x = Some(vec![42, 10, 12]);
    /// let mut y = 0;
    /// let mut z = vec![];
    /// for i in 0..5 {
    ///     if let Some(x) = x.some_mut() {
    ///         y += 2;
    ///         if y >= 4 {
    ///             z = x.take();
    ///         }
    ///     } else {
    ///         z.push(i)
    ///     }
    /// }
    ///
    /// assert!(x.is_none());
    /// assert_eq!(y, 4);
    /// assert_eq!(z, [42, 10, 12, 2, 3, 4])
    /// ```
    pub fn take(self) -> T {
        // SAFETY: safety invariant on SomeMut
        unsafe { self.0.take().unwrap_unchecked() }
    }

    /// Unwrap this `SomeMut` into a normal mutable reference, with a lifetime tied to the original option.
    pub fn into_mut(self) -> &'a mut T {
        // SAFETY: safety invariant on SomeMut
        unsafe { self.0.as_mut().unwrap_unchecked() }
    }

    /// Unwrap this `SomeMut` into a mutable reference to the original option.
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
