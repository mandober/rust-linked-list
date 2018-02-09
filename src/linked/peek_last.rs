use linked::list::*;


impl<T> List<T> {

  /// Returns a reference to the payload of the list's last node
  pub fn peek_last(&self) -> Option<&T> {
    match self.size() {
      0 => None,
      1 => self.peek(),
      _ => {
        let mut link = self.head.as_ref();

        while let Some(boxed_node) = link {
          if let None = boxed_node.next {
            return link.map(|node| &node.payload);
          }
          link = boxed_node.next.as_ref();
        };

        unimplemented!()
      }
    }
  } //fn
} //impl
