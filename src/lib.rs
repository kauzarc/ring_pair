//! Tiny ring-buffer types specialized for exactly two elements.
//!
//! This crate provides:
//! - [`RingPair`], storing values inline as `[T; 2]`
//! - [`BoxedRingPair`], storing values on the heap as `Box<[T; 2]>`
//!
//! # Examples
//! ```rust
//! use ring_pair::{BoxedRingPair, RingPair};
//!
//! let mut inline = RingPair::new(1);
//! inline.push(2);
//! assert_eq!(inline.as_pair(), (&1, &2));
//!
//! let mut boxed = BoxedRingPair::new(String::from("a"));
//! boxed.push(String::from("b"));
//! assert_eq!(boxed.as_pair(), (&String::from("a"), &String::from("b")));
//! ```

mod inner;
mod ring_pair;
mod boxed_ring_pair;

pub use ring_pair::RingPair;
pub use boxed_ring_pair::BoxedRingPair;
