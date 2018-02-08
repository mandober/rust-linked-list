use linked::list::*;

impl<T> List<T> {

  /// Puts a payload in a new node and prepends it onto the list.
  pub fn push(&mut self, payload: T) {
    let new_node = Box::new(Node {
      payload,
      next: self.head.take(),
    });
    self.head = Some(new_node);
    self.len += 1;
  }



  pub fn push_commented(&mut self, payload: T) {
    // The goal is:
    // Create a new node.
    // Set the `next` field of the new node to point to the old list;
    // Old list, like any list, is represented by a List struct that has
    // a `head` field which holds a Link enum that has a None variant if
    // no first node or Some variant pointing to the first node in list.
    // We want that link. When we get that link, we can assign it to the
    // next field of the new node:
    //   next: self.head
    // Than we assign the new node to head field:
    //   self.head = new_node
    //
    // push mutates the list, so it takes it as `&mut self`. This is the
    // first requirement that makes the list (or any part of it) mutable.
    // The other requirement is a mut ref to head (self.head) field.
    //
    // first, we create a new node to store the payload
    let new_boxed_node = Box::new(Node {
      //
      // payload: the easy part
      payload: payload,
      //
      // if we could do just this, it would be easy:
      //   next: self.head
      // error: CANNOT MOVE OUT OF BORROWED CONTENT
      //
      // We need to steal that link that head is holding, but the only wat
      // to do it is to put something back (of the same type). Since link
      // is enum Link, it is easy to figure out what to put back: a None.
      //
      // This switcharoo can be done with `mem::replace` function.
      //   pub fn std::mem::replace<T>(dest: &mut T, src: T) -> T
      // Replaces the value at a mutable location with a new one,
      // returning the old value.
      //
      // next: ::std::mem::replace(&mut self.head, None)
      //
      // We replace self.head link temporarily with None link. In the next
      // line we'll change it again and put this new node in None's place.
      //
      // This is so common that Option has a `take` method that does the same:
      //   fn take(&mut self) -> Option<T>
      // Takes the value out of Option, leaving a None in its place.
      //
      // thus:
      next: self.head.take()
    });

    // Due to recent maneuvre, `self.head` contains None.
    // `self` was taken from the start as &mut self - with
    // that requirement met we can change its `head` field:
    self.head = Some(new_boxed_node);

    // Inc the size of the list:
    self.len += 1;
  }

} //impl