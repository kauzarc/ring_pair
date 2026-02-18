//! Fixed-size circular buffer of exactly 2 elements with boxed storage.
//!
//! # Examples
//! ```rust
//! use ring_pair::BoxedRingPair;
//!
//! let mut pair = BoxedRingPair::new(10);
//! pair.push(20);
//! pair.push(30);
//!
//! assert_eq!(pair.as_pair(), (&20, &30));
//! ```

use core::fmt;
use core::hash::{Hash, Hasher};
use std::boxed::Box;

/// Circular buffer holding exactly 2 elements with boxed storage and O(1) push.
#[derive(Clone, Default)]
pub struct BoxedRingPair<T> {
    buffer: Box<[T; 2]>,
    newest: usize,
}

impl<T: Clone> BoxedRingPair<T> {
    /// Creates a new `BoxedRingPair` with both slots initialized to the given value.
    ///
    /// # Examples
    /// ```rust
    /// use ring_pair::BoxedRingPair;
    ///
    /// let pair = BoxedRingPair::new("x");
    /// assert_eq!(pair.as_pair(), (&"x", &"x"));
    /// ```
    pub fn new(initial: T) -> Self {
        Self {
            buffer: Box::new([initial.clone(), initial]),
            newest: 0,
        }
    }
}

impl<T> BoxedRingPair<T> {
    /// Pushes a new value, making it the newest.
    ///
    /// # Examples
    /// ```rust
    /// use ring_pair::BoxedRingPair;
    ///
    /// let mut pair = BoxedRingPair::new(1);
    /// pair.push(2);
    ///
    /// assert_eq!(pair.older(), &1);
    /// assert_eq!(pair.newer(), &2);
    /// ```
    pub fn push(&mut self, value: T) {
        self.newest = 1 - self.newest;
        self.buffer[self.newest] = value;
    }

    /// Advances to the next slot and allows in-place initialization.
    ///
    /// # Examples
    /// ```rust
    /// use ring_pair::BoxedRingPair;
    ///
    /// let mut pair = BoxedRingPair::from((String::from("old"), String::from("stale")));
    /// pair.push_with(|slot| {
    ///     slot.clear();
    ///     slot.push_str("new");
    /// });
    ///
    /// assert_eq!(pair.as_pair(), (&String::from("stale"), &String::from("new")));
    /// ```
    pub fn push_with<F>(&mut self, f: F)
    where
        F: FnOnce(&mut T),
    {
        self.newest = 1 - self.newest;
        f(&mut self.buffer[self.newest]);
    }

    /// Returns a reference to the newer element.
    pub fn newer(&self) -> &T {
        &self.buffer[self.newest]
    }

    /// Returns a reference to the older element.
    pub fn older(&self) -> &T {
        &self.buffer[1 - self.newest]
    }

    /// Returns both elements as `(older, newer)`.
    pub fn as_pair(&self) -> (&T, &T) {
        (self.older(), self.newer())
    }
}

impl<T: fmt::Debug> fmt::Debug for BoxedRingPair<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoxedRingPair")
            .field("older", self.older())
            .field("newer", self.newer())
            .finish()
    }
}

impl<T: PartialEq> PartialEq for BoxedRingPair<T> {
    fn eq(&self, other: &Self) -> bool {
        self.older() == other.older() && self.newer() == other.newer()
    }
}

impl<T: Eq> Eq for BoxedRingPair<T> {}

impl<T: Hash> Hash for BoxedRingPair<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.older().hash(state);
        self.newer().hash(state);
    }
}

impl<T> From<[T; 2]> for BoxedRingPair<T> {
    /// Creates a `BoxedRingPair` from `[older, newer]`.
    ///
    /// # Examples
    /// ```rust
    /// use ring_pair::BoxedRingPair;
    ///
    /// let pair = BoxedRingPair::from([3, 4]);
    /// assert_eq!(pair.as_pair(), (&3, &4));
    /// ```
    fn from([older, newer]: [T; 2]) -> Self {
        Self {
            buffer: Box::new([newer, older]),
            newest: 0,
        }
    }
}

impl<T> From<(T, T)> for BoxedRingPair<T> {
    /// Creates a `BoxedRingPair` from `(older, newer)`.
    ///
    /// # Examples
    /// ```rust
    /// use ring_pair::BoxedRingPair;
    ///
    /// let pair = BoxedRingPair::from(("older", "newer"));
    /// assert_eq!(pair.as_pair(), (&"older", &"newer"));
    /// ```
    fn from((older, newer): (T, T)) -> Self {
        Self {
            buffer: Box::new([newer, older]),
            newest: 0,
        }
    }
}
