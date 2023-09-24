# Ribbon - tape for Iterators

`Ribbon` is meant to provide API for holding (and thus making available) some
number of items returned by iterators.

This is meant for cases where using iterators is convenient, but some context
around the item returned by an iterator is needed. This is especially useful
when look-ahead is necessary (we need to know what values come after the current
one before deciding what to do with it).

This crate provides two types that implement the `Ribbon` trait:

- `Tape`: a dynamically sized `Ribbon` that can hold varying number of items and
  can grow and shrink as necessary. It is backed up by a `VecDeque`, and
  allocates memory on the heap (as is customary by dynamically sized
  collections)
- `Band`: a fix-sized `Ribbon` backed up by an array of `N` elements. It cannot
  grow over the given fixed length, and instead drops the first element if no
  space is available at the given moment.

## Examples:

### Using `Tape`

```rust
use ribbon::Tape;

let mut tape = Tape::new(0..10);
tape.expand_n(5);

assert_eq!(tape.len(), 5);
assert_eq!(tape.peek_front(), Some(&0));
assert_eq!(tape.peek_back(), Some(&4));
```

### Using `Band`

```rust
use ribbon::Band;

// Band with capacity for 5 items
let mut band: Band<3, _, _> = Band::new(0..4);
band.expand_n(2); // consume 0, 1 from iterator

assert_eq!(band.len(), 2);
assert_eq!(band.peek_front(), Some(&0));
assert_eq!(band.peek_back(), Some(&1));

// just expands, no need to pop first item
assert_eq!(band.progress(), None); // consume 3 from iterator

// needs space, pops first item
assert_eq!(band.progress(), Some(0)); // consumes 4 from iterator, iterator has no more values

// iterator does not produce more values, progress becomes no-op. No extra capacity is needed,
// hence progress does not return any more values.
assert_eq!(band.progress(), None);
```
