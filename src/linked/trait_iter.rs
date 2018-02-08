use linked::list::*;

// first, a struct that iterator will return
// IntoIter struct: just a wrapper over List
pub struct IntoIter<T>(List<T>);

// `into_iter` List method that returns the wrapping struct on iteration.
// It consumes `self` i.e. moves the list instance
impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

// implement Iterator trait for the wrapping struct IntoIter
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}



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
