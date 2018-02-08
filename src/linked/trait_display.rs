use linked::list::*;

use std::fmt::{self, Display};

impl<T> Display for List<T>
  where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cur_link = &self.head;

        write!(f, "List: ")?;
        while let &Some(ref boxed_node) = cur_link {
            write!(f, "[{}] -> ", boxed_node.payload)?;
            cur_link = &boxed_node.next;
        }
        write!(f, "nil")
    }

} // impl
