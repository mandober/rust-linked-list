use linked::list::*;

impl<T> List<T> {

  /// Returns a reference to the payload of the list's head node
  pub fn peek(&self) -> Option<&T> {
    self.head.as_ref().map(|node| &node.payload)
  }



  pub fn peek_typed(&self) -> Option<&T> {
    // self.head : Option<Box<Node<T>>>
    let first_link: &Option<Box<Node<T>>> = &self.head;
    let first_link_as_ref: Option<&Box<Node<T>>> = first_link.as_ref();
    let opt_ref_payload: Option<&T> = first_link_as_ref.map(|ref_boxed_node: &Box<Node<T>>| {
      let ref_payload: &T = &ref_boxed_node.payload;
      ref_payload
    });
    opt_ref_payload
  }

  pub fn peek_commented(&self) -> Option<&T> {
    self.head       // : Option<Box<Node<T>>>
      // as_ref: Option<T> weakens T  to Option<&T>, thus:
      // as_ref: Option<Box<Node<T>>> to Option<&Box<Node<T>>>
      .as_ref()   // : Option<&Box<Node<T>>>
      // Maps Option<T> to Option<U> by applying fn to contained value (T)
      // here, contained value is &Box<Node<T>>
      .map(|node| {   // : &Box<Node<T>>
          &node.payload  // : Option<&T>
      })
  }


} //impl
