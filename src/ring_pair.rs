//! Fixed-size circular buffer of exactly 2 elements.
//!
//! # Examples
//! ```rust
//! use ring_pair::RingPair;
//!
//! let mut pair = RingPair::new(10);
//! pair.push(20);
//! pair.push(30);
//!
//! assert_eq!(pair.as_pair(), (&20, &30));
//! ```

use core::fmt;
use core::hash::{Hash, Hasher};

#[cfg(feature = "alloc")]
use crate::boxed_ring_pair::BoxedRingPair;
use crate::inner::RingPairInner;

/// Circular buffer holding exactly 2 elements with O(1) push.
#[derive(Clone, Copy, Default)]
pub struct RingPair<T> {
    pub(crate) inner: RingPairInner<[T; 2]>,
}

impl<T: Clone> RingPair<T> {
    /// Creates a new `RingPair` with both slots initialized to the given value.
    ///
    /// # Examples
    /// ```rust
    /// use ring_pair::RingPair;
    ///
    /// let pair = RingPair::new("x");
    /// assert_eq!(pair.as_pair(), (&"x", &"x"));
    /// ```
    pub fn new(initial: T) -> Self {
        Self {
            inner: RingPairInner {
                buffer: [initial.clone(), initial],
                newest: false,
            },
        }
    }
}

impl<T> RingPair<T> {
    /// Pushes a new value, making it the newest.
    ///
    /// # Examples
    /// ```rust
    /// use ring_pair::RingPair;
    ///
    /// let mut pair = RingPair::new(1);
    /// pair.push(2);
    ///
    /// assert_eq!(pair.older(), &1);
    /// assert_eq!(pair.newer(), &2);
    /// ```
    pub fn push(&mut self, value: T) {
        self.inner.push(value);
    }

    /// Advances to the next slot and returns a mutable reference to it,
    /// allowing in-place initialization without copying.
    ///
    /// # Examples
    /// ```rust
    /// use ring_pair::RingPair;
    ///
    /// let mut pair = RingPair::from((String::from("old"), String::from("stale")));
    /// pair.push_with(|slot| {
    ///     slot.clear();
    ///     slot.push_str("new");
    /// });
    ///
    /// assert_eq!(pair.as_pair(), (&String::from("stale"), &String::from("new")));
    /// ```
    pub fn push_with<F: FnOnce(&mut T)>(&mut self, f: F) {
        self.inner.push_with(f);
    }

    /// Returns a reference to the newer element.
    pub fn newer(&self) -> &T {
        self.inner.newer()
    }

    /// Returns a reference to the older element.
    pub fn older(&self) -> &T {
        self.inner.older()
    }

    /// Returns both elements as `(older, newer)`.
    pub fn as_pair(&self) -> (&T, &T) {
        (self.inner.older(), self.inner.newer())
    }

    /// Returns an iterator over the elements, from oldest to newest.
    ///
    /// # Examples
    /// ```rust
    /// use ring_pair::RingPair;
    ///
    /// let mut pair = RingPair::new(1);
    /// pair.push(2);
    /// pair.push(3);
    ///
    /// let collected: Vec<_> = pair.iter().collect();
    /// assert_eq!(collected, [&2, &3]);
    /// ```
    #[must_use]
    pub fn iter(&self) -> crate::Iter<'_, T> {
        crate::Iter::new(self.inner.older(), self.inner.newer())
    }
}

impl<T: fmt::Debug> fmt::Debug for RingPair<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RingPair")
            .field("older", self.inner.older())
            .field("newer", self.inner.newer())
            .finish()
    }
}

impl<T: PartialEq> PartialEq for RingPair<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T: Eq> Eq for RingPair<T> {}

impl<T: Hash> Hash for RingPair<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<T> From<[T; 2]> for RingPair<T> {
    /// Creates a `RingPair` from `[older, newer]`.
    ///
    /// # Examples
    /// ```rust
    /// use ring_pair::RingPair;
    ///
    /// let pair = RingPair::from([3, 4]);
    /// assert_eq!(pair.as_pair(), (&3, &4));
    /// ```
    fn from([older, newer]: [T; 2]) -> Self {
        Self {
            inner: RingPairInner {
                buffer: [newer, older],
                newest: false,
            },
        }
    }
}

impl<T> From<(T, T)> for RingPair<T> {
    /// Creates a `RingPair` from `(older, newer)`.
    ///
    /// # Examples
    /// ```rust
    /// use ring_pair::RingPair;
    ///
    /// let pair = RingPair::from(("older", "newer"));
    /// assert_eq!(pair.as_pair(), (&"older", &"newer"));
    /// ```
    fn from((older, newer): (T, T)) -> Self {
        Self {
            inner: RingPairInner {
                buffer: [newer, older],
                newest: false,
            },
        }
    }
}

#[cfg(feature = "alloc")]
impl<T> From<BoxedRingPair<T>> for RingPair<T> {
    /// Moves a `BoxedRingPair` onto the stack.
    ///
    /// # Examples
    /// ```rust
    /// use ring_pair::{BoxedRingPair, RingPair};
    ///
    /// let boxed = BoxedRingPair::from((1, 2));
    /// let inline = RingPair::from(boxed);
    /// assert_eq!(inline.as_pair(), (&1, &2));
    /// ```
    fn from(pair: BoxedRingPair<T>) -> Self {
        Self {
            inner: RingPairInner {
                buffer: *pair.inner.buffer,
                newest: pair.inner.newest,
            },
        }
    }
}
