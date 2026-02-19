//! Tiny ring-buffer types specialized for exactly two elements.
//!
//! This crate provides:
//! - [`RingPair`], storing values inline as `[T; 2]`
//! - [`BoxedRingPair`], storing values on the heap as `Box<[T; 2]>` (requires the `alloc` feature)
//!
//! # `no_std` support
//!
//! This crate is `no_std` compatible. [`RingPair`] is always available.
//! [`BoxedRingPair`] requires the `alloc` feature (enabled by default).
//!
//! # Examples
//! ```rust
//! use ring_pair::RingPair;
//!
//! let mut inline = RingPair::new(1);
//! inline.push(2);
//! assert_eq!(inline.as_pair(), (&1, &2));
//! ```
//!
//! ```rust
//! # #[cfg(feature = "alloc")]
//! use ring_pair::BoxedRingPair;
//!
//! # #[cfg(feature = "alloc")]
//! let mut boxed = BoxedRingPair::new(String::from("a"));
//! # #[cfg(feature = "alloc")]
//! boxed.push(String::from("b"));
//! # #[cfg(feature = "alloc")]
//! assert_eq!(boxed.as_pair(), (&String::from("a"), &String::from("b")));
//! ```

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod boxed_ring_pair;
mod inner;
mod iter;
mod ring_pair;

#[cfg(feature = "alloc")]
pub use boxed_ring_pair::BoxedRingPair;
pub use iter::Iter;
pub use ring_pair::RingPair;
