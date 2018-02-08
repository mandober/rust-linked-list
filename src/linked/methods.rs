use linked::list::*;

impl<T> List<T> {
    /// Makes a new empty list.
    pub fn new() -> Self {
        List {
            head: None,
            len: 0,
        }
    }

    /// Get list size
    pub fn size(&self) -> i32 {
        // list size is usize, but cast it as i32
        *&self.len as i32
    }

    /// Check if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Removes all elements from the list.
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    /*
    /// Check if the list contains a given element
    pub fn contains(&self, x: &T) -> bool
        where T: PartialEq<T>
    {
        self.iter().any(|e| e == x)
    }
    */


}
