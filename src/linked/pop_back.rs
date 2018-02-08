use linked::list::*;

impl<T> List<T> where T: ::std::fmt::Debug {

  pub fn pop_back(&mut self) {//-> Option<T> {
    match self.size() {
      0 => {}//{return None;}
      1 => {self.pop();}
      _ => {
        let mut link = self.head
                           .as_mut()
                           .map(|node| {
                              node.next
                                  .as_mut()
                                  .map(|node| &mut *node)
                           });

        while let Some(Some(mb_node)) = link {
          // we need second to last node
          //if let None = mb_node.next.as_mut().map(|node| &mut node.next) {
          if let None = mb_node.next {
            //mb_node is second to last
            println!("second to last, mb_node: {:?}", *mb_node);
            break;
          }
          link = mb_node.next.as_mut()
                             .map(|node| {
                                Some(&mut *node)
                             });
        }
      }

    } //match

  } //fn




  pub fn pops(&mut self) -> Option<T> {
    self.head
        .take()
        .map(|boxed_node| {
          let node = *boxed_node;
          self.head = node.next;
          self.len -= 1;
          node.payload
        })
  }

}