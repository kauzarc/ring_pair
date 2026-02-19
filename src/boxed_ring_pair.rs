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

use crate::inner::RingPairInner;
use crate::ring_pair::RingPair;

/// Circular buffer holding exactly 2 elements with boxed storage and O(1) push.
///
/// The two elements live on the heap, so moving a `BoxedRingPair<T>` is always
/// cheap regardless of the size of `T`. Prefer this over [`RingPair`] when `T`
/// is large enough that keeping two copies on the stack is undesirable (e.g.
/// large arrays, big structs). For small `T`, prefer [`RingPair`] to avoid the
/// heap allocation.
///
/// [`RingPair`]: crate::RingPair
#[derive(Clone, Default)]
pub struct BoxedRingPair<T> {
    pub(crate) inner: RingPairInner<Box<[T; 2]>>,
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
            inner: RingPairInner {
                buffer: Box::new([initial.clone(), initial]),
                newest: false,
            },
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
        self.inner.push(value);
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
    pub fn push_with<F: FnOnce(&mut T)>(&mut self, f: F) {
        self.inner.push_with(f);
    }

    /// Returns a reference to the newer element.
    #[expect(clippy::must_use_candidate, reason = "pure getter; ignoring the result is never a useful pattern")]
    pub fn newer(&self) -> &T {
        self.inner.newer()
    }

    /// Returns a reference to the older element.
    #[expect(clippy::must_use_candidate, reason = "pure getter; ignoring the result is never a useful pattern")]
    pub fn older(&self) -> &T {
        self.inner.older()
    }

    /// Returns both elements as `(older, newer)`.
    #[expect(clippy::must_use_candidate, reason = "pure getter; ignoring the result is never a useful pattern")]
    pub fn as_pair(&self) -> (&T, &T) {
        (self.inner.older(), self.inner.newer())
    }
}

impl<T: fmt::Debug> fmt::Debug for BoxedRingPair<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoxedRingPair")
            .field("older", self.inner.older())
            .field("newer", self.inner.newer())
            .finish()
    }
}

impl<T: PartialEq> PartialEq for BoxedRingPair<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T: Eq> Eq for BoxedRingPair<T> {}

impl<T: Hash> Hash for BoxedRingPair<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
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
            inner: RingPairInner { buffer: Box::new([newer, older]), newest: false },
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
            inner: RingPairInner { buffer: Box::new([newer, older]), newest: false },
        }
    }
}

impl<T> From<RingPair<T>> for BoxedRingPair<T> {
    /// Moves a `RingPair` onto the heap.
    ///
    /// # Examples
    /// ```rust
    /// use ring_pair::{BoxedRingPair, RingPair};
    ///
    /// let inline = RingPair::from((1, 2));
    /// let boxed = BoxedRingPair::from(inline);
    /// assert_eq!(boxed.as_pair(), (&1, &2));
    /// ```
    fn from(pair: RingPair<T>) -> Self {
        Self { inner: RingPairInner { buffer: Box::new(pair.inner.buffer), newest: pair.inner.newest } }
    }
}
