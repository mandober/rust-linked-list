use core::fmt;
use core::mem;

use core::cmp::Ordering;
use core::hash::{Hasher, Hash};
use core::iter::{FromIterator, FusedIterator};
use core::marker::PhantomData;
use core::ops::{BoxPlace, InPlace, Place, Placer};
use core::ptr::{self, Shared};
use boxed::{Box, IntermediateBox};
use super::SpecExtend;


/// A doubly-linked list with owned nodes.
///
/// The `LinkedList` allows pushing and popping elements at either end
/// in constant time.
pub struct LinkedList<T> {
    head: Option<Shared<Node<T>>>,
    tail: Option<Shared<Node<T>>>,
    len: usize,
    marker: PhantomData<Box<Node<T>>>,
}

struct Node<T> {
    next: Option<Shared<Node<T>>>,
    prev: Option<Shared<Node<T>>>,
    element: T,
}



/// An iterator over the elements of a `LinkedList`.
pub struct Iter<'a, T: 'a> {
    head: Option<Shared<Node<T>>>,
    tail: Option<Shared<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

impl<'a, T: 'a + fmt::Debug> fmt::Debug for Iter<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Iter")
         .field(&self.len)
         .finish()
    }
}

impl<'a, T> Clone for Iter<'a, T> {
    fn clone(&self) -> Self {
        Iter { ..*self }
    }
}


/// A mutable iterator over the elements of a `LinkedList`.
pub struct IterMut<'a, T: 'a> {
    list: &'a mut LinkedList<T>,
    head: Option<Shared<Node<T>>>,
    tail: Option<Shared<Node<T>>>,
    len: usize,
}

impl<'a, T: 'a + fmt::Debug> fmt::Debug for IterMut<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("IterMut")
         .field(&self.list)
         .field(&self.len)
         .finish()
    }
}


/// An owning iterator over the elements of a `LinkedList`.
#[derive(Clone)]
pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T: fmt::Debug> fmt::Debug for IntoIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("IntoIter")
         .field(&self.list)
         .finish()
    }
}


impl<T> Node<T> {
    fn new(element: T) -> Self {
        Node {
            next: None,
            prev: None,
            element,
        }
    }

    fn into_element(self: Box<Self>) -> T {
        self.element
    }
}


// private methods
impl<T> LinkedList<T> {

    /// Adds the given node to the front of the list.
    fn push_front_node(&mut self, mut node: Box<Node<T>>) {
        unsafe {
            node.next = self.head;
            node.prev = None;
            let node = Some(Shared::from(Box::into_unique(node)));

            match self.head {
                None => self.tail = node,
                Some(mut head) => head.as_mut().prev = node,
            }

            self.head = node;
            self.len += 1;
        }
    }

    /// Removes and returns the node at the front of the list.
    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;

            match self.head {
                None => self.tail = None,
                Some(mut head) => head.as_mut().prev = None,
            }

            self.len -= 1;
            node
        })
    }

    /// Adds the given node to the back of the list.
    fn push_back_node(&mut self, mut node: Box<Node<T>>) {
        unsafe {
            node.next = None;
            node.prev = self.tail;
            let node = Some(Shared::from(Box::into_unique(node)));

            match self.tail {
                None => self.head = node,
                Some(mut tail) => tail.as_mut().next = node,
            }

            self.tail = node;
            self.len += 1;
        }
    }

    /// Removes and returns the node at the back of the list.
    fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
        self.tail.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.tail = node.prev;

            match self.tail {
                None => self.head = None,
                Some(mut tail) => tail.as_mut().next = None,
            }

            self.len -= 1;
            node
        })
    }
}




impl<T> Default for LinkedList<T> {
    /// Creates an empty `LinkedList<T>`.
    fn default() -> Self {
        Self::new()
    }
}


impl<T> LinkedList<T> {
    /// Creates an empty `LinkedList`.
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
            marker: PhantomData,
        }
    }

    /// Moves all elements from `other` to the end of the list.
    /// This reuses all the nodes from `other` and moves them into `self`. After
    /// this operation, `other` becomes empty.
    pub fn append(&mut self, other: &mut Self) {
        match self.tail {
            None => mem::swap(self, other),
            Some(mut tail) => {
                if let Some(mut other_head) = other.head.take() {
                    unsafe {
                        tail.as_mut().next = Some(other_head);
                        other_head.as_mut().prev = Some(tail);
                    }

                    self.tail = other.tail.take();
                    self.len += mem::replace(&mut other.len, 0);
                }
            }
        }
    }

    /// Provides a forward iterator.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
            marker: PhantomData,
        }
    }

    /// Provides a forward iterator with mutable references.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            len: self.len,
            list: self,
        }
    }

    /// Returns `true` if the `LinkedList` is empty.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Returns the length of the `LinkedList`.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Removes all elements from the `LinkedList`.
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    /// Returns `true` if the `LinkedList` contains an element equal to the
    /// given value.
    pub fn contains(&self, x: &T) -> bool
        where T: PartialEq<T>
       {
        self.iter().any(|e| e == x)
    }

    /// Provides a reference to the front element, or `None` if the list is
    /// empty.
    pub fn front(&self) -> Option<&T> {
        unsafe {
            self.head.as_ref().map(|node| &node.as_ref().element)
        }
    }

    /// Provides a mutable reference to the front element, or `None` if the list
    /// is empty.
    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.head.as_mut().map(|node| &mut node.as_mut().element)
        }
    }

    /// Provides a reference to the back element, or `None` if the list is
    /// empty.
    pub fn back(&self) -> Option<&T> {
        unsafe {
            self.tail.as_ref().map(|node| &node.as_ref().element)
        }
    }

    /// Provides a mutable reference to the back element, or `None` if the list
    /// is empty.
    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.tail.as_mut().map(|node| &mut node.as_mut().element)
        }
    }

    /// Adds an element first in the list.
    pub fn push_front(&mut self, elt: T) {
        self.push_front_node(box Node::new(elt));
    }

    /// Removes the first element and returns it, or `None` if the list is
    /// empty.
    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(Node::into_element)
    }

    /// Appends an element to the back of a list
    pub fn push_back(&mut self, elt: T) {
        self.push_back_node(box Node::new(elt));
    }

    /// Removes the last element from a list and returns it, or `None` if
    /// it is empty.
    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node().map(Node::into_element)
    }

    /// Splits the list into two at the given index. Returns everything after the given index,
    /// including the index.
    pub fn split_off(&mut self, at: usize) -> LinkedList<T> {
        let len = self.len();
        assert!(at <= len, "Cannot split off at a nonexistent index");
        if at == 0 {
            return mem::replace(self, Self::new());
        } else if at == len {
            return Self::new();
        }

        // Below, we iterate towards the `i-1`th node, either from the start or the end,
        // depending on which would be faster.
        let split_node = if at - 1 <= len - 1 - (at - 1) {
            let mut iter = self.iter_mut();
            // instead of skipping using .skip() (which creates a new struct),
            // we skip manually so we can access the head field without
            // depending on implementation details of Skip
            for _ in 0..at - 1 {
                iter.next();
            }
            iter.head
        } else {
            // better off starting from the end
            let mut iter = self.iter_mut();
            for _ in 0..len - 1 - (at - 1) {
                iter.next_back();
            }
            iter.tail
        };

        // The split node is the new tail node of the first part and owns
        // the head of the second part.
        let second_part_head;

        unsafe {
            second_part_head = split_node.unwrap().as_mut().next.take();
            if let Some(mut head) = second_part_head {
                head.as_mut().prev = None;
            }
        }

        let second_part = LinkedList {
            head: second_part_head,
            tail: self.tail,
            len: len - at,
            marker: PhantomData,
        };

        // Fix the tail ptr of the first part
        self.tail = split_node;
        self.len = at;

        second_part
    }

    /// Returns a place for insertion at the front of the list.
    #[unstable(feature = "collection_placement",
               reason = "method name and placement protocol are subject to change",
               issue = "30172")]
    pub fn front_place(&mut self) -> FrontPlace<T> {
        FrontPlace {
            list: self,
            node: IntermediateBox::make_place(),
        }
    }

    /// Returns a place for insertion at the back of the list.
    #[unstable(feature = "collection_placement",
               reason = "method name and placement protocol are subject to change",
               issue = "30172")]
    pub fn back_place(&mut self) -> BackPlace<T> {
        BackPlace {
            list: self,
            node: IntermediateBox::make_place(),
        }
    }
}


unsafe impl<#[may_dangle] T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front_node() {}
    }
}


impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                // Need an unbound lifetime to get 'a
                let node = &*node.as_ptr();
                self.len -= 1;
                self.head = node.next;
                &node.element
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}


impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a T> {
        if self.len == 0 {
            None
        } else {
            self.tail.map(|node| unsafe {
                // Need an unbound lifetime to get 'a
                let node = &*node.as_ptr();
                self.len -= 1;
                self.tail = node.prev;
                &node.element
            })
        }
    }
}


impl<'a, T> ExactSizeIterator for Iter<'a, T> {}

#[unstable(feature = "fused", issue = "35602")]
impl<'a, T> FusedIterator for Iter<'a, T> {}


impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<&'a mut T> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| unsafe {
                // Need an unbound lifetime to get 'a
                let node = &mut *node.as_ptr();
                self.len -= 1;
                self.head = node.next;
                &mut node.element
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}


impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut T> {
        if self.len == 0 {
            None
        } else {
            self.tail.map(|node| unsafe {
                // Need an unbound lifetime to get 'a
                let node = &mut *node.as_ptr();
                self.len -= 1;
                self.tail = node.prev;
                &mut node.element
            })
        }
    }
}


impl<'a, T> ExactSizeIterator for IterMut<'a, T> {}

#[unstable(feature = "fused", issue = "35602")]
impl<'a, T> FusedIterator for IterMut<'a, T> {}

impl<'a, T> IterMut<'a, T> {
    /// Inserts the given element just after the element most recently returned by `.next()`.
    /// The inserted element does not appear in the iteration.
    #[inline]
    #[unstable(feature = "linked_list_extras",
               reason = "this is probably better handled by a cursor type -- we'll see",
               issue = "27794")]
    pub fn insert_next(&mut self, element: T) {
        match self.head {
            None => self.list.push_back(element),
            Some(mut head) => unsafe {
                let mut prev = match head.as_ref().prev {
                    None => return self.list.push_front(element),
                    Some(prev) => prev,
                };

                let node = Some(Shared::from(Box::into_unique(box Node {
                    next: Some(head),
                    prev: Some(prev),
                    element,
                })));

                prev.as_mut().next = node;
                head.as_mut().prev = node;

                self.list.len += 1;
            },
        }
    }

    /// Provides a reference to the next element, without changing the iterator.
    #[inline]
    #[unstable(feature = "linked_list_extras",
               reason = "this is probably better handled by a cursor type -- we'll see",
               issue = "27794")]
    pub fn peek_next(&mut self) -> Option<&mut T> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                self.head.as_mut().map(|node| &mut node.as_mut().element)
            }
        }
    }
}


impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len, Some(self.list.len))
    }
}


impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.list.pop_back()
    }
}


impl<T> ExactSizeIterator for IntoIter<T> {}

#[unstable(feature = "fused", issue = "35602")]
impl<T> FusedIterator for IntoIter<T> {}


impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}


impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Consumes the list into an iterator yielding elements by value.
    #[inline]
    fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }
}


impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}


impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}


impl<T> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        <Self as SpecExtend<I>>::spec_extend(self, iter);
    }
}

impl<I: IntoIterator> SpecExtend<I> for LinkedList<I::Item> {
    default fn spec_extend(&mut self, iter: I) {
        for elt in iter {
            self.push_back(elt);
        }
    }
}

impl<T> SpecExtend<LinkedList<T>> for LinkedList<T> {
    fn spec_extend(&mut self, ref mut other: LinkedList<T>) {
        self.append(other);
    }
}

#[stable(feature = "extend_ref", since = "1.2.0")]
impl<'a, T: 'a + Copy> Extend<&'a T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }
}


impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }

    fn ne(&self, other: &Self) -> bool {
        self.len() != other.len() || self.iter().ne(other)
    }
}


impl<T: Eq> Eq for LinkedList<T> {}


impl<T: PartialOrd> PartialOrd for LinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }
}


impl<T: Ord> Ord for LinkedList<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other)
    }
}


impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        self.iter().cloned().collect()
    }
}


impl<T: fmt::Debug> fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}


impl<T: Hash> Hash for LinkedList<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for elt in self {
            elt.hash(state);
        }
    }
}

unsafe fn finalize<T>(node: IntermediateBox<Node<T>>) -> Box<Node<T>> {
    let mut node = node.finalize();
    ptr::write(&mut node.next, None);
    ptr::write(&mut node.prev, None);
    node
}

/// A place for insertion at the front of a `LinkedList`.
#[must_use = "places do nothing unless written to with `<-` syntax"]
#[unstable(feature = "collection_placement",
           reason = "struct name and placement protocol are subject to change",
           issue = "30172")]
pub struct FrontPlace<'a, T: 'a> {
    list: &'a mut LinkedList<T>,
    node: IntermediateBox<Node<T>>,
}

#[unstable(feature = "collection_placement",
           reason = "struct name and placement protocol are subject to change",
           issue = "30172")]
impl<'a, T: 'a + fmt::Debug> fmt::Debug for FrontPlace<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("FrontPlace")
         .field(&self.list)
         .finish()
    }
}

#[unstable(feature = "collection_placement",
           reason = "placement protocol is subject to change",
           issue = "30172")]
impl<'a, T> Placer<T> for FrontPlace<'a, T> {
    type Place = Self;

    fn make_place(self) -> Self {
        self
    }
}

#[unstable(feature = "collection_placement",
           reason = "placement protocol is subject to change",
           issue = "30172")]
impl<'a, T> Place<T> for FrontPlace<'a, T> {
    fn pointer(&mut self) -> *mut T {
        unsafe { &mut (*self.node.pointer()).element }
    }
}

#[unstable(feature = "collection_placement",
           reason = "placement protocol is subject to change",
           issue = "30172")]
impl<'a, T> InPlace<T> for FrontPlace<'a, T> {
    type Owner = ();

    unsafe fn finalize(self) {
        let FrontPlace { list, node } = self;
        list.push_front_node(finalize(node));
    }
}

/// A place for insertion at the back of a `LinkedList`.
#[must_use = "places do nothing unless written to with `<-` syntax"]
#[unstable(feature = "collection_placement",
           reason = "struct name and placement protocol are subject to change",
           issue = "30172")]
pub struct BackPlace<'a, T: 'a> {
    list: &'a mut LinkedList<T>,
    node: IntermediateBox<Node<T>>,
}

#[unstable(feature = "collection_placement",
           reason = "struct name and placement protocol are subject to change",
           issue = "30172")]
impl<'a, T: 'a + fmt::Debug> fmt::Debug for BackPlace<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("BackPlace")
         .field(&self.list)
         .finish()
    }
}

#[unstable(feature = "collection_placement",
           reason = "placement protocol is subject to change",
           issue = "30172")]
impl<'a, T> Placer<T> for BackPlace<'a, T> {
    type Place = Self;

    fn make_place(self) -> Self {
        self
    }
}

#[unstable(feature = "collection_placement",
           reason = "placement protocol is subject to change",
           issue = "30172")]
impl<'a, T> Place<T> for BackPlace<'a, T> {
    fn pointer(&mut self) -> *mut T {
        unsafe { &mut (*self.node.pointer()).element }
    }
}

#[unstable(feature = "collection_placement",
           reason = "placement protocol is subject to change",
           issue = "30172")]
impl<'a, T> InPlace<T> for BackPlace<'a, T> {
    type Owner = ();

    unsafe fn finalize(self) {
        let BackPlace { list, node } = self;
        list.push_back_node(finalize(node));
    }
}

// Ensure that `LinkedList` and its read-only iterators are covariant in their type parameters.
#[allow(dead_code)]
fn assert_covariance() {
    fn a<'a>(x: LinkedList<&'static str>) -> LinkedList<&'a str> {
        x
    }
    fn b<'i, 'a>(x: Iter<'i, &'static str>) -> Iter<'i, &'a str> {
        x
    }
    fn c<'a>(x: IntoIter<&'static str>) -> IntoIter<&'a str> {
        x
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}

unsafe impl<T: Sync> Sync for LinkedList<T> {}

unsafe impl<'a, T: Sync> Send for Iter<'a, T> {}

unsafe impl<'a, T: Sync> Sync for Iter<'a, T> {}

unsafe impl<'a, T: Send> Send for IterMut<'a, T> {}

unsafe impl<'a, T: Sync> Sync for IterMut<'a, T> {}