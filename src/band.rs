//! Implementation of statically sized data structures that implement the [`Ribbon`] trait.

use crate::{ribbon, Ribbon};

/// A fix-sized [`Ribbon`] backed up by an array of `N` elements. It cannot grow over the given
/// fixed length, and instead drops and/or returns items if no space is available at the given
/// moment.
///
/// [`Ribbon`]: crate::Ribbon
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Band<const LEN: usize, I>
where
    I: Iterator,
{
    iter: I,
    tape: [Option<I::Item>; LEN],
    head: usize,
    len: usize,
}

impl<const LEN: usize, I> Band<LEN, I>
where
    I: Iterator,
{
    /// Creates a new `Tape` from the given iterator.
    pub fn new(iter: I) -> Band<LEN, I> {
        let tape = [0; LEN].map(|_| None);

        Band {
            iter,
            tape,
            head: 0,
            len: 0,
        }
    }

    /// Shifts all items by 1, returning the head of the `Band`.
    ///
    /// Shifting is a misnomer, and runs in `O(1)`. Rather than shifting elements, the indices
    /// pointing to the first and last element are shifted.
    fn slide(&mut self) -> Option<I::Item> {
        let first = self.tape[self.head].take()?;

        self.incr_head();
        self.len = self.len.saturating_sub(1);

        Some(first)
    }

    /// Checks if the `Band` is at full capacity.
    fn is_full(&self) -> bool {
        self.len() == LEN
    }

    /// Moves the head index by 1, wrapping around to the start of inner array when longer than
    /// `LEN`.
    fn incr_head(&mut self) {
        self.head = (self.head + 1) % LEN;
    }

    /// Calculates the tail index based on head index and length of the `Band`.
    fn tail(&self) -> usize {
        (self.head + self.len.saturating_sub(1)) % LEN
    }
}

impl<const LEN: usize, I> ribbon::Ribbon<I::Item> for Band<LEN, I>
where
    I: Iterator,
{
    fn progress(&mut self) -> Option<I::Item> {
        let next = self.iter.next()?; // do nothing if iterator does not produce

        let head = self.slide();
        self.len += 1;

        self.tape[self.tail()] = Some(next);
        head
    }

    /// Expands the `Band` by consuming the next available item and appending it to the end.
    /// Drops the first element if the `Band` is already at full capacity.
    fn expand(&mut self) {
        if self.is_full() {
            self.slide();
        } else {
            self.tape[self.len] = self.iter.next();
            self.len += 1;
        }
    }

    fn pop_front(&mut self) -> Option<I::Item> {
        self.slide()
    }

    fn peek_front(&self) -> Option<&I::Item> {
        self.peek_at(0)
    }

    fn peek_front_mut(&mut self) -> Option<&mut I::Item> {
        self.peek_at_mut(0)
    }

    fn pop_back(&mut self) -> Option<I::Item> {
        let back = self.tape[self.tail()].take()?;
        self.len -= 1;
        Some(back)
    }

    fn peek_back(&self) -> Option<&I::Item> {
        self.peek_at(self.tail())
    }

    fn peek_back_mut(&mut self) -> Option<&mut I::Item> {
        self.peek_at_mut(self.tail())
    }

    fn peek_at(&self, index: usize) -> Option<&I::Item> {
        if index >= LEN {
            return None;
        }

        let idx = (self.head + index) % LEN;
        self.tape.get(idx)?.as_ref()
    }

    fn peek_at_mut(&mut self, index: usize) -> Option<&mut I::Item> {
        if index >= LEN {
            return None;
        }

        let idx = (self.head + index) % LEN;
        self.tape.get_mut(idx)?.as_mut()
    }

    fn len(&self) -> usize {
        self.len
    }
}

impl<const LEN: usize, I> Iterator for Band<LEN, I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            self.expand_n(LEN);
        }

        self.pop_front()
    }
}

impl<const LEN: usize, I> From<I> for Band<LEN, I>
where
    I: Iterator,
{
    fn from(value: I) -> Self {
        Band::new(value)
    }
}

impl<const LEN: usize, I> Clone for Band<LEN, I>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
            tape: self.tape.clone(),
            head: self.head,
            len: self.len,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Band;
    use crate::{ribbon::Ribbon, Enroll};

    #[test]
    fn expands() {
        let mut band: Band<5, _> = Band::new(0u32..10u32);

        assert_eq!(band.peek_front(), None);
        assert_eq!(band.peek_back(), None);

        band.expand();
        assert_eq!(band.peek_front(), Some(&0));
        assert_eq!(band.peek_back(), Some(&0));

        band.expand();
        assert_eq!(band.peek_front(), Some(&0));
        assert_eq!(band.peek_back(), Some(&1));

        band.expand_n(3);
        assert_eq!(band.peek_front(), Some(&0));
        assert_eq!(band.peek_back(), Some(&4));
    }

    #[test]
    fn pops_front() {
        let mut band: Band<5, _> = Band::new(0u32..10u32);
        band.expand_n(5);

        assert_eq!(band.pop_front(), Some(0));
        assert_eq!(band.pop_front(), Some(1));
        assert_eq!(band.pop_front(), Some(2));
        assert_eq!(band.pop_front(), Some(3));
        assert_eq!(band.pop_front(), Some(4));
        assert_eq!(band.pop_front(), None);
    }

    #[test]
    fn pops_back() {
        let mut band: Band<5, _> = Band::new(0u32..10u32);
        dbg!(&band);
        band.expand_n(5);
        dbg!(&band);

        assert_eq!(band.pop_back(), Some(4));
        dbg!(&band);
        assert_eq!(band.pop_back(), Some(3));
        assert_eq!(band.pop_back(), Some(2));
        assert_eq!(band.pop_back(), Some(1));
        assert_eq!(band.pop_back(), Some(0));
        assert_eq!(band.pop_back(), None);
    }

    #[test]
    fn peeks_at() {
        let mut band: Band<5, _> = Band::new(0u32..10u32);
        band.expand_n(5);

        assert_eq!(band.peek_at(0), Some(&0));
        assert_eq!(band.peek_at(1), Some(&1));
        assert_eq!(band.peek_at(2), Some(&2));
        assert_eq!(band.peek_at(3), Some(&3));
        assert_eq!(band.peek_at(4), Some(&4));
        assert_eq!(band.peek_at(5), None);
    }

    #[test]
    fn len_correct() {
        let mut band: Band<5, _> = Band::new(0u32..10u32);
        band.expand_n(5);

        assert_eq!(band.len(), 5);

        band.pop_back();
        assert_eq!(band.len(), 4);

        band.pop_back();
        assert_eq!(band.len(), 3);

        band.pop_back();
        assert_eq!(band.len(), 2);

        band.pop_back();
        assert_eq!(band.len(), 1);

        band.pop_back();
        assert_eq!(band.len(), 0);
    }

    #[test]
    fn makes_progress() {
        let mut band: Band<5, _> = Band::new(0u32..5u32);

        // band was empty, first progress has nothing to return
        assert_eq!(band.progress(), None);

        // progresses 1 by 1 item, this can be observed as simple pass-through of the underlying
        // iterator
        assert_eq!(band.progress(), Some(0));
        assert_eq!(band.progress(), Some(1));
        assert_eq!(band.progress(), Some(2));
        assert_eq!(band.progress(), Some(3));

        // iterator does not produce more values, so progress does not drop anything
        assert_eq!(band.progress(), None);
    }

    #[test]
    fn peek_back() {
        let mut band = (0..10).band::<5>();

        assert_eq!(band.len(), 0);
        band.expand_n(5);

        assert_eq!(band.len(), 5);
        assert_eq!(band.peek_back(), Some(&4));

        if let Some(item) = band.peek_back_mut() {
            *item = 42;
        }

        assert_eq!(band.peek_back(), Some(&42));
    }

    #[test]
    fn is_iterator() {
        let mut band = (0..10).band::<5>();

        assert_eq!(band.next(), Some(0));
        assert_eq!(band.next(), Some(1));
        assert_eq!(band.next(), Some(2));
        assert_eq!(band.next(), Some(3));
        assert_eq!(band.next(), Some(4));
        assert_eq!(band.next(), Some(5));
    }
}
