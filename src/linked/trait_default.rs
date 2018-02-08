use linked::list::*;

impl<T> Default for List<T> {
    /// Creates an empty list.
    fn default() -> Self {
        Self::new()
    }
}
