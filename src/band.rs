//! Implementation of statically sized data structures that implement the [`Ribbon`] trait.

use crate::{ribbon, Ribbon};

/// A fix-sized [`Ribbon`] backed up by an array of `N` elements. It cannot grow over the given fixed
/// length, and instead drops and/or returns items if no space is available at the given moment.
///
/// [`Ribbon`]: crate::Ribbon
#[derive(Debug)]
pub struct Band<const N: usize, I, T> {
    iter: I,
    tape: [Option<T>; N],
}

impl<const LEN: usize, I, T> Band<LEN, I, T> {
    /// Creates a new `Tape` from the given iterator.
    pub fn new(iter: I) -> Band<LEN, I, T>
    where
        I: Iterator<Item = T>,
        T: Sized,
    {
        let tape = [0; LEN].map(|_| None);
        Band { iter, tape }
    }

    /// Shifts all items by 1, returning the head of the `Band`.
    fn slide(&mut self) -> Option<T> {
        let first = self.tape[0].take();

        for i in 1..LEN {
            self.tape[i - 1] = self.tape[i].take();
        }

        first
    }

    /// Checks if the `Band` is at full capacity.
    fn is_full(&self) -> bool
    where
        I: Iterator<Item = T>,
    {
        self.peek_at(LEN - 1).is_some()
    }
}

impl<const LEN: usize, I, T> ribbon::Ribbon<T> for Band<LEN, I, T>
where
    I: Iterator<Item = T>,
{
    fn progress(&mut self) -> Option<T> {
        let next = self.iter.next()?; // do nothing if iterator does not produce

        let head = self.slide();
        self.tape[self.len()] = Some(next);
        head
    }

    /// Expands the `Band` by consuming the next available item and appending it to the end.
    /// Drops the first element if the `Band` is already at full capacity.
    fn expand(&mut self) {
        if self.is_full() {
            self.slide();
        } else {
            self.tape[self.len()] = self.iter.next();
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        let first = self.tape[0].take();
        self.slide();

        first
    }

    fn peek_front(&self) -> Option<&T> {
        self.peek_at(0)
    }

    fn pop_back(&mut self) -> Option<T> {
        let idx = self.len().saturating_sub(1);
        self.tape[idx].take()
    }

    fn peek_back(&self) -> Option<&T> {
        let idx = self.len().saturating_sub(1);
        self.peek_at(idx)
    }

    fn peek_at(&self, index: usize) -> Option<&T> {
        self.tape.get(index)?.as_ref()
    }

    fn len(&self) -> usize {
        self.tape.iter().position(|x| x.is_none()).unwrap_or(LEN)
    }
}

#[cfg(test)]
mod tests {
    use super::Band;
    use crate::ribbon::Ribbon;

    #[test]
    fn expands() {
        let mut band: Band<5, _, _> = Band::new(0u32..10u32);

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
        let mut band: Band<5, _, _> = Band::new(0u32..10u32);
        band.expand_n(5);

        assert_eq!(band.pop_front(), Some(0));
        dbg!(&band);
        assert_eq!(band.pop_front(), Some(1));
        assert_eq!(band.pop_front(), Some(2));
        assert_eq!(band.pop_front(), Some(3));
        assert_eq!(band.pop_front(), Some(4));
        assert_eq!(band.pop_front(), None);
    }

    #[test]
    fn pops_back() {
        let mut band: Band<5, _, _> = Band::new(0u32..10u32);
        band.expand_n(5);

        assert_eq!(band.pop_back(), Some(4));
        assert_eq!(band.pop_back(), Some(3));
        assert_eq!(band.pop_back(), Some(2));
        assert_eq!(band.pop_back(), Some(1));
        assert_eq!(band.pop_back(), Some(0));
        assert_eq!(band.pop_back(), None);
    }

    #[test]
    fn peeks_at() {
        let mut band: Band<5, _, _> = Band::new(0u32..10u32);
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
        let mut band: Band<5, _, _> = Band::new(0u32..10u32);
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
        let mut band: Band<5, _, _> = Band::new(0u32..5u32);

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
}
