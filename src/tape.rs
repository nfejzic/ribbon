use std::collections::VecDeque;

/// A tape is a data structure that can be expanded by pulling items from an iterator, and provides
/// a way to access some dynamic number of items produced by the iterator at the same time.
#[derive(Debug)]
pub struct Tape<I, T> {
    iter: I,
    tape: VecDeque<T>,
}

impl<I, T> Tape<I, T> {
    /// Creates a new `Tape` from the given iterator.
    pub fn new(iter: I) -> Tape<I, T>
    where
        I: Iterator<Item = T>,
    {
        Tape {
            iter,
            tape: VecDeque::new(),
        }
    }
}

impl<I, T> super::ribbon::Ribbon<T> for Tape<I, T>
where
    I: Iterator<Item = T>,
{
    fn expand(&mut self) {
        if let Some(item) = self.iter.next() {
            self.tape.push_back(item);
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        self.tape.pop_front()
    }

    fn peek_front(&self) -> Option<&T> {
        self.tape.front()
    }

    fn pop_back(&mut self) -> Option<T> {
        self.tape.pop_back()
    }

    fn peek_back(&self) -> Option<&T> {
        self.tape.back()
    }

    fn peek_at(&self, index: usize) -> Option<&T> {
        self.tape.get(index)
    }

    fn len(&self) -> usize {
        self.tape.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::ribbon::Ribbon;
    use crate::tape::Tape;

    #[test]
    fn expands() {
        let mut tape = Tape::new(0u32..10u32);

        assert_eq!(tape.peek_front(), None);
        assert_eq!(tape.peek_back(), None);

        tape.expand();
        assert_eq!(tape.peek_front(), Some(&0));
        assert_eq!(tape.peek_back(), Some(&0));

        tape.expand();
        assert_eq!(tape.peek_front(), Some(&0));
        assert_eq!(tape.peek_back(), Some(&1));
    }

    #[test]
    fn pops_front() {
        let mut tape = Tape::new(0..10);
        tape.expand_n(5);

        assert_eq!(tape.pop_front(), Some(0));
        assert_eq!(tape.pop_front(), Some(1));
        assert_eq!(tape.pop_front(), Some(2));
        assert_eq!(tape.pop_front(), Some(3));
        assert_eq!(tape.pop_front(), Some(4));
        assert_eq!(tape.pop_front(), None);
    }

    #[test]
    fn pops_back() {
        let mut tape = Tape::new(0..10);
        tape.expand_n(5);

        assert_eq!(tape.pop_back(), Some(4));
        assert_eq!(tape.pop_back(), Some(3));
        assert_eq!(tape.pop_back(), Some(2));
        assert_eq!(tape.pop_back(), Some(1));
        assert_eq!(tape.pop_back(), Some(0));
        assert_eq!(tape.pop_back(), None);
    }

    #[test]
    fn peeks_at() {
        let mut tape = Tape::new(0..10);
        tape.expand_n(5);

        assert_eq!(tape.peek_at(0), Some(&0));
        assert_eq!(tape.peek_at(1), Some(&1));
        assert_eq!(tape.peek_at(2), Some(&2));
        assert_eq!(tape.peek_at(3), Some(&3));
        assert_eq!(tape.peek_at(4), Some(&4));
        assert_eq!(tape.peek_at(5), None);
    }

    #[test]
    fn len_correct() {
        let mut tape = Tape::new(0..10);
        tape.expand_n(5);

        assert_eq!(tape.len(), 5);

        tape.pop_back();
        assert_eq!(tape.len(), 4);

        tape.pop_back();
        assert_eq!(tape.len(), 3);

        tape.pop_back();
        assert_eq!(tape.len(), 2);

        tape.pop_back();
        assert_eq!(tape.len(), 1);

        tape.pop_back();
        assert_eq!(tape.len(), 0);
    }
}
