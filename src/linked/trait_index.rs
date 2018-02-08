

pub trait Index<Idx> where Idx: ?Sized {
    type Output: ?Sized;
    fn index(&self, index: Idx) -> &Self::Output;
}

use std::ops::Index;
impl<T> Index<Link<T>> for List<T> {
    type Output = Link<T>;
    fn index(&self, index: usize) -> &Self::Output {
       &self.head
    }
}
