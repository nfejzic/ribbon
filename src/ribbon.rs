pub trait Ribbon<T> {
    /// Tries to move `Ribbon` forward through the iterator without expanding itself. Returns the
    /// first element if `Ribbon` is at full capacity, since it would be dropped by driving the
    /// `Ribbon` forward. See [`Band`] for an example of a `Ribbon` with fixed capacity.
    ///
    /// [`Band`]: crate::band::Band
    fn progress(&mut self) -> Option<T> {
        self.expand();
        self.pop_front()
    }

    /// Expands the `Ribbon` by consuming the next available item and appending it to the end.
    fn expand(&mut self);

    /// Expands the `Ribbon` by consuming the `n` next available item and appending it to the end.
    fn expand_n(&mut self, n: usize) {
        for _ in 0..n {
            self.expand();
        }
    }

    /// Removes the item stored at the head of `Ribbon` and returns it (if available).
    fn pop_front(&mut self) -> Option<T>;

    /// Returns a reference to the item stored at the head of `Ribbon` and returns it (if
    /// available).
    fn peek_front(&self) -> Option<&T> {
        self.peek_at(0)
    }

    /// Removes the item stored at the tail of `Ribbon` and returns it (if available).
    fn pop_back(&mut self) -> Option<T>;

    /// Returns a reference to the item stored at the tail of `Ribbon` and returns it (if
    /// available).
    fn peek_back(&self) -> Option<&T> {
        self.peek_at(self.len() - 1)
    }

    /// Returns a reference to the item stored at the given index of `Ribbon` and returns it (if
    /// available). Returns `None` if index out of bounds.
    fn peek_at(&self, index: usize) -> Option<&T>;

    /// Returns the number of items currently found on the `Ribbon`.
    fn len(&self) -> usize;

    /// Returns `true` if `Ribbon` does not contain any items at the moment.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
