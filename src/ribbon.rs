use crate::{Band, Tape};

pub trait Ribbon<T> {
    /// Tries to stream the iterator forward through the `Ribbon` without expanding it. Underlying
    /// iterator is polled for the next element. Returns the head of the `Ribbon`, and the new item
    /// from the iterator is appended to the tail.
    ///
    /// Is a no-op if iterator stops producing values. In that case `None` is returned.
    ///
    /// # Example
    ///
    /// ```
    /// use ribbon::{Ribbon, Tape};
    ///
    /// let mut tape = Tape::new(0..10);
    ///
    /// tape.expand_n(5);
    /// let item = tape.progress();
    ///
    /// assert_eq!(item, Some(0));
    /// assert_eq!(tape.len(), 5);
    /// assert_eq!(tape.peek_front(), Some(&1));
    /// assert_eq!(tape.peek_back(), Some(&5));
    /// ```
    fn progress(&mut self) -> Option<T>;

    /// Expands the `Ribbon` by consuming the next available item and appending it to the tail.
    ///
    /// # Example
    ///
    ///```
    /// use ribbon::{Ribbon, Tape};
    ///
    /// let mut tape = Tape::new(0..10);
    ///
    /// tape.expand();
    /// assert_eq!(tape.len(), 1);
    /// assert_eq!(tape.peek_front(), Some(&0));
    /// assert_eq!(tape.peek_back(), Some(&0));
    ///
    /// tape.expand();
    /// assert_eq!(tape.len(), 2);
    /// assert_eq!(tape.peek_front(), Some(&0));
    /// assert_eq!(tape.peek_back(), Some(&1));
    /// ```
    fn expand(&mut self);

    /// Expands the `Ribbon` by consuming the `n` next available item and appending it to the end.
    ///
    /// # Example
    ///
    ///```
    /// use ribbon::{Ribbon, Tape};
    ///
    /// let mut tape = Tape::new(0..10);
    ///
    /// tape.expand_n(5);
    /// assert_eq!(tape.len(), 5);
    /// assert_eq!(tape.peek_front(), Some(&0));
    /// assert_eq!(tape.peek_back(), Some(&4));
    ///
    /// tape.expand_n(7);
    /// assert_eq!(tape.len(), 10);
    /// assert_eq!(tape.peek_front(), Some(&0));
    /// assert_eq!(tape.peek_back(), Some(&9));
    /// ```
    fn expand_n(&mut self, n: usize) {
        for _ in 0..n {
            self.expand();
        }
    }

    /// Removes the item stored at the head of `Ribbon` and returns it (if available).
    ///
    /// # Example
    ///
    /// ```rust
    /// use ribbon::{Ribbon, Tape};
    ///
    /// let mut tape = Tape::new(0..10);
    ///
    /// tape.expand_n(2);
    /// assert_eq!(tape.len(), 2);
    /// assert_eq!(tape.pop_front(), Some(0));
    /// assert_eq!(tape.pop_front(), Some(1));
    /// assert_eq!(tape.pop_front(), None);
    /// ```
    fn pop_front(&mut self) -> Option<T>;

    /// Returns a reference to the item stored at the head of `Ribbon` and returns it (if
    /// available).
    ///
    /// # Example
    ///
    /// ```rust
    /// use ribbon::{Ribbon, Tape};
    ///
    /// let mut tape = Tape::new(0..10);
    ///
    /// tape.expand_n(2);
    /// assert_eq!(tape.len(), 2);
    /// assert_eq!(tape.peek_front(), Some(&0));
    /// assert_eq!(tape.len(), 2);
    /// ```
    fn peek_front(&self) -> Option<&T> {
        self.peek_at(0)
    }

    /// Removes the item stored at the tail of `Ribbon` and returns it (if available).
    ///
    /// # Example
    ///
    /// ```rust
    /// use ribbon::{Ribbon, Tape};
    ///
    /// let mut tape = Tape::new(0..10);
    ///
    /// tape.expand_n(3);
    /// assert_eq!(tape.len(), 3);
    /// assert_eq!(tape.pop_back(), Some(2));
    /// assert_eq!(tape.pop_back(), Some(1));
    /// assert_eq!(tape.pop_back(), Some(0));
    /// assert_eq!(tape.pop_back(), None);
    /// ```
    fn pop_back(&mut self) -> Option<T>;

    /// Returns a reference to the item stored at the tail of `Ribbon` and returns it (if
    /// available).
    ///
    /// # Example
    ///
    /// ```rust
    /// use ribbon::{Ribbon, Tape};
    ///
    /// let mut tape = Tape::new(0..10);
    ///
    /// tape.expand_n(3);
    /// assert_eq!(tape.len(), 3);
    /// assert_eq!(tape.peek_back(), Some(&2));
    ///
    /// tape.expand();
    /// assert_eq!(tape.peek_back(), Some(&3));
    ///
    /// tape.expand();
    /// assert_eq!(tape.peek_back(), Some(&4));
    /// ```
    fn peek_back(&self) -> Option<&T> {
        self.peek_at(self.len() - 1)
    }

    /// Returns a reference to the item stored at the given index of `Ribbon` and returns it (if
    /// available). Returns `None` if index out of bounds.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ribbon::{Ribbon, Tape};
    ///
    /// let mut tape = Tape::new(0..10);
    ///
    /// tape.expand_n(5);
    /// assert_eq!(tape.len(), 5);
    /// assert_eq!(tape.peek_at(0), Some(&0));
    /// assert_eq!(tape.peek_at(2), Some(&2));
    /// assert_eq!(tape.peek_at(3), Some(&3));
    /// ```
    fn peek_at(&self, index: usize) -> Option<&T>;

    /// Returns the number of items currently found on the `Ribbon`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ribbon::{Ribbon, Tape};
    ///
    /// let mut tape = Tape::new(0..10);
    ///
    /// tape.expand_n(5);
    /// assert_eq!(tape.len(), 5);
    ///
    /// tape.expand_n(2);
    /// assert_eq!(tape.len(), 7);
    /// ```
    fn len(&self) -> usize;

    /// Returns `true` if `Ribbon` does not contain any items at the moment.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ribbon::{Ribbon, Tape};
    ///
    /// let mut tape = Tape::new(0..10);
    /// assert_eq!(tape.len(), 0);
    ///
    /// tape.expand();
    /// assert_eq!(tape.len(), 1);
    ///
    /// tape.expand_n(5);
    /// assert_eq!(tape.len(), 6);
    /// ```
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Extension trait on types that implement [`Iterator`] trait with convenient functions to convert
/// the given [`Iterator`] into a [`Band`] or [`Tape`].
///
/// [`Band`]: crate::Band
/// [`Tape`]: crate::Tape
pub trait Enroll {
    /// Creates a new [`Band`] from the given Iterator.
    ///
    /// [`Band`]: crate::Band
    fn band<const N: usize>(self) -> crate::Band<N, Self>
    where
        Self: Sized + Iterator;

    /// Creates a new [`Tape`] from the given Iterator.
    ///
    /// [`Tape`]: crate::Tape
    fn tape(self) -> crate::Tape<Self>
    where
        Self: Sized + Iterator;
}

impl<I> Enroll for I
where
    I: Iterator,
{
    fn band<const N: usize>(self) -> Band<N, Self>
    where
        Self: Sized + Iterator,
    {
        crate::Band::<N, Self>::new(self)
    }

    fn tape(self) -> Tape<Self>
    where
        Self: Sized + Iterator,
    {
        crate::Tape::new(self)
    }
}
