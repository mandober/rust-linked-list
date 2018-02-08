use linked::list::*;


impl<T> List<T> {

    pub fn peek_last_mut(&mut self, _elem: T) {

        //           : &Option<Box<Node<T>>>
        let mut link = &self.head;

        while let &Some(ref boxed_node) = link {

            match boxed_node.next {
                None => {
                    println!("{:?}", *boxed_node);
                    break;
                },
                Some(_) => {}
            }

            if let None = boxed_node.next {
                // boxed_node:  &Box<Node<T>
                // link: &Option<Box<Node<T>>>

                let r = boxed_node;
                println!("{:?}", r);

                //println!("{:?}", *boxed_node);
                // Node { payload: 1, next: None }

                // println!("{:?}", boxed_node.next);
                // None

                // println!("{:?}", *link);
                // Some(Node { payload: 1, next: None })

                //return (*boxed_node).as_mut().map(|node| &mut node.payload);

                break;
            }
            link = &boxed_node.next;
        }

    }


} // impl
