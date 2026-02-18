//! Fixed-size circular buffer of exactly 2 elements.

use core::fmt;
use core::hash::{Hash, Hasher};

/// Circular buffer holding exactly 2 elements with O(1) push.
#[derive(Clone, Copy, Default)]
pub struct RingPair<T> {
    buffer: [T; 2],
    newest: usize,
}

impl<T: Copy> RingPair<T> {
    /// Creates a new `RingPair` with both slots initialized to the given value.
    pub fn new(initial: T) -> Self {
        Self {
            buffer: [initial; 2],
            newest: 0,
        }
    }
}

impl<T> RingPair<T> {
    /// Pushes a new value, making it the newest.
    pub fn push(&mut self, value: T) {
        self.newest = 1 - self.newest;
        self.buffer[self.newest] = value;
    }

    /// Advances to the next slot and returns a mutable reference to it,
    /// allowing in-place initialization without copying.
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

impl<T: fmt::Debug> fmt::Debug for RingPair<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RingPair")
            .field("older", self.older())
            .field("newer", self.newer())
            .finish()
    }
}

impl<T: PartialEq> PartialEq for RingPair<T> {
    fn eq(&self, other: &Self) -> bool {
        self.older() == other.older() && self.newer() == other.newer()
    }
}

impl<T: Eq> Eq for RingPair<T> {}

impl<T: Hash> Hash for RingPair<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.older().hash(state);
        self.newer().hash(state);
    }
}

impl<T> From<[T; 2]> for RingPair<T> {
    /// Creates a `RingPair` from `[older, newer]`.
    fn from([older, newer]: [T; 2]) -> Self {
        Self {
            buffer: [newer, older],
            newest: 0,
        }
    }
}

impl<T> From<(T, T)> for RingPair<T> {
    /// Creates a `RingPair` from `(older, newer)`.
    fn from((older, newer): (T, T)) -> Self {
        Self {
            buffer: [newer, older],
            newest: 0,
        }
    }
}
