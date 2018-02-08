use linked::list::*;

impl<T> Drop for List<T> {
    /// Drops the list recursively by node.
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }

}
