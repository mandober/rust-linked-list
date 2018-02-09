use linked::list::*;

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.payload
        })
    }
}


/*
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> { /* stuff */ }
}

Which can be desugarred to:

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next<'b>(&'b mut self) -> Option<&'a T> { /* stuff */ }
}
*/




// ITERATORS
// Creating your own iterator for some collection is 2 steps process:
//  1. create a (wrapping) struct to hold the iterator's state
//  2. implement Iterator trait for that (wrapping) struct
//
// traits:
/*
    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    pub trait IntoIterator
      where <Self::IntoIter as Iterator>::Item == Self::Item
    {
        type Item;
        type IntoIter: Iterator;
        fn into_iter(self) -> Self::IntoIter;
    }
*/
