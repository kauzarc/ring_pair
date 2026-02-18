# ring_pair

Tiny ring-buffer types specialized for exactly two elements.

## Types

- `RingPair<T>`: inline storage with `[T; 2]`
- `BoxedRingPair<T>`: heap storage with `Box<[T; 2]>`

Both provide O(1) `push`, `push_with`, and accessors for older/newer entries.

## Example

```rust
use ring_pair::{BoxedRingPair, RingPair};

let mut inline = RingPair::new(1);
inline.push(2);
assert_eq!(inline.as_pair(), (&1, &2));

let mut boxed = BoxedRingPair::new(String::from("a"));
boxed.push(String::from("b"));
assert_eq!(boxed.as_pair(), (&String::from("a"), &String::from("b")));
```

## Development

```bash
cargo test
```
