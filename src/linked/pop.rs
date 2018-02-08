use linked::list::*;


impl<T> List<T> {

  /// Puts a payload in a new node and prepends it onto the list.
  pub fn pop(&mut self) -> Option<T> {
    self.head
        .take()
        .map(|boxed_node| {
          let node = *boxed_node;
          self.head = node.next;
          self.len -= 1;
          node.payload
        })
  }



  pub fn pop_commented2(&mut self) -> Option<T> {

    //       : Option<Box<Node<T>>>
    let taken = self.head
                    .take()        // Option<Box<Node<T>>>
                    .map(|boxed_node| {   // Box<Node<T>>
                      let node = *boxed_node; // Node<T>
                      self.head = node.next;
                      node.payload
                    });
    // dec the size
    self.len -= 1;

    // return
    taken
  }


  pub fn pop_commented1(&mut self) -> Option<T> {

    match ::std::mem::replace(&mut self.head, None) {
      // replace returns the type Option<Box<Node<T>>>
      // so either Some(_) or None

      // list is empty
      None => None,

      // list has at least first node
      Some(boxed_node) => {     //        Box<Node<T>>

        // Pull the node out of the box, so we can disect it easily.
        // We do that by dereferencing the box, which unboxes the node
        // i.e. it places the node on the stack.
        let node = *boxed_node; //           <Node<T>

        // The original second link, now becomes first
        self.head = node.next;  // Option<Box<Node<T>>>

        // dec the size
        self.len -= 1;

        // return optianl payload
        Some(node.payload)      // Some(T)
      }
    }

  }

} //impl