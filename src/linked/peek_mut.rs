use linked::list::*;

impl<T> List<T> {

  /// Returns a mutable reference to the payload of the list's head node
  pub fn peek_mut(&mut self) -> Option<&mut T> {
    self.head.as_mut().map(|node| &mut node.payload)
  }



  pub fn peek_mut_commented(&mut self) -> Option<&mut T> {
    self.head       // Option<Box<Node<T>>>
      .as_mut()   // Option<&mut Box<Node<T>>>
      .map(|node| {       // : &mut Box<Node<T>>
          &mut node.payload  // : Option<&mut T>
      })
  }


} //impl
