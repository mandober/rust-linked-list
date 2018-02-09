use linked::list::*;
use std::ops::Index;

/*
pub trait Index<Idx> where Idx: ?Sized {
    type Output: ?Sized;
    fn index(&self, index: Idx) -> &Self::Output;
}
*/

// impl<T> Index for List<T> {

impl<T> Index<Link<T>> for List<T> {
    type Output = Link<T>;
    fn index(&self, index: usize) -> &Self::Output {
       &self.head
    }
}
