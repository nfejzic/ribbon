use crate::ribbon;

/// A tape is a data structure that can be expanded by pulling items from an iterator, and provides
/// a way to access some dynamic number of items produced by the iterator at the same time.
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

    /// Shifts all items by 1, overwriting the first item in the `Band`.
    fn slide(&mut self) {
        for i in 1..LEN {
            self.tape[i - 1] = self.tape[i].take();
        }
    }
}

impl<const LEN: usize, I, T> ribbon::Ribbon<T> for Band<LEN, I, T>
where
    I: Iterator<Item = T>,
{
    fn progress(&mut self) -> Option<T> {
        let first = self.tape[0].take();
        self.expand();
        first
    }

    /// Expands the `Ribbon` by consuming the next available item and appending it to the end.
    /// Drops the first element if the `Band` is already at full capacity.
    fn expand(&mut self) {
        if let Some(item) = self.iter.next() {
            if self.peek_at(LEN - 1).is_some() {
                self.slide()
            }

            self.tape[self.len()] = Some(item);
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
        let mut band: Band<5, _, _> = Band::new(0u32..10u32);

        // Band was empty, nothing returned
        assert_eq!(band.progress(), None);
        assert_eq!(band.progress(), Some(0));
        assert_eq!(band.progress(), Some(1));
        assert_eq!(band.progress(), Some(2));
        assert_eq!(band.progress(), Some(3));
        assert_eq!(band.progress(), Some(4));
        assert_eq!(band.progress(), Some(5));
        assert_eq!(band.progress(), Some(6));
        assert_eq!(band.progress(), Some(7));
        assert_eq!(band.progress(), Some(8));
        assert_eq!(band.progress(), Some(9));
        assert_eq!(band.progress(), None);
        assert_eq!(band.progress(), None);
    }
}
