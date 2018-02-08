use linked::list::*;

impl<T> List<T> {
  /// Puts a payload in a new node and appends it onto the list.
  pub fn append(&mut self, payload: T) {
    if self.is_empty() {
      self.push(payload);
      return;
    }

    let mut link = self.head.as_mut();
    while let Some(mut_boxed_node) = link {
      if let None = mut_boxed_node.next {
        let new_node = Some(Box::new(Node { payload, next: None }));
        mut_boxed_node.next = new_node;
        self.len += 1;
        break;
      }
      link = mut_boxed_node.next.as_mut();
    }

  }



  pub fn append_by_box(&mut self, payload: T) {
    // let mut link: Option<        Box<Node<T>>> = self.head;
    //
    // let mut link: Option<&mut    Box<Node<T>>> = self.head.as_mut();
    // let mut link: Option<&'e mut Box<Node<T>>> = self.head.as_mut().map(|node| &mut *node);
    // let mut link: Option<&'e mut     Node<T>>  = self.head.as_mut().map(|node| &mut **node);
    // let mut link: Option<&'e mut     Node<T>>  = self.head.map(|node| &mut *node); // map eats the box
    //
    //
    //      link: Option<&mut Box<Node<T>>>
    let mut link = self.head.as_mut().map(|node| &mut *node);
    //                                          Option<Box<Node<T>>>
    //                      .as_mut()   // Option<&mut Box<Node<T>>>
    //                   .map(|node|    //        &mut Box<Node<T>>
    //                     &mut *node   //             Box<Node<T>>
    //                     &mut **node  //                <Node<T>
    //                   );
    //
    // append needs to mutate the list, so it takes it as `&mut self`.
    // That's the first requirement, but it's not enough: to change the
    // last link so it points to the new node we're about to create, we
    // need a mutable ref to the last box (boxed node). Than it's just:
    //   boxed_node.next = new_node;
    //
    // So we start with an `Option<Box<Node<T>>>`,
    // but need to arrive at `&mut Box<Node<T>>`,
    //
    // Mutable variable `link` holds an optional mutable reference
    // to the current (boxed) node.
    //
    while let Some(boxed_node) = link {
      // if this is the last node
      if let None = boxed_node.next {
        let new_node = Some(Box::new(Node { payload, next: None }));
        // and finally:
        boxed_node.next = new_node;
        self.len += 1;
        break;
      }
      link = boxed_node.next.as_mut().map(|node| &mut *node);
    }
    // end
  }

  pub fn append_naked_node(&mut self, payload: T) {
      let mut link = self.head.as_mut().map(|node| &mut **node);
      while let Some(mut_node) = link {
          // if this is the last node
          if let None = mut_node.next {
              let new_node = Some(Box::new(Node { payload, next: None }));
              mut_node.next = new_node;
              self.len += 1;
              break;
          }
      link = mut_node.next.as_mut().map(|node| &mut **node);
      }
  }

}