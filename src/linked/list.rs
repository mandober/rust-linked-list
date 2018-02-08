// FILE: src/linked/list.rs
// DESC: linked list definition

/// Link is a type alias for `enum Option<Box<Node<T>>>`.
/// It represents two possibilities for a link:
/// either `Some(Box<Node<T>>)` a boxed Node (representing a cons)
/// or a `None` (representing a nill)
type Link<T> = Option<Box<Node<T>>>;

/// `struct List` represents a list itself. It has a `head` filed
/// which points to first node in the actual list. The field `len`
/// contains the size of the list.
#[derive(Debug)]
pub struct List<T> {
    pub head: Link<T>,
    pub len: usize,
}

/// `struct Node` represents a list's node with a `payload` field
/// and a `next` field that points to the next node (or null).
#[derive(Debug)]
pub struct Node<T> {
    pub payload: T,
    pub next: Link<T>
}

// list with 2 nodes (3 links)
//
//      head: Link         next: Link         next: Link
// List ----------> [Node] ----------> [Node] ----------> ∅
//        Some(_)            Some(_)              None


// List struct is not a node, it just represents the list by pointing
// to the first node with its head field (of type Link). In case of
// empty list head is None.
//
// Node<T> struct is the meat, it carries a payload (type T)
// and it has a `next` field of type Link.
// Node must be allocated, therefore it is boxed, Box<Node<T>>
//
// Link is an alias for Option<Box<Node<T>>>
// Link is either Some(boxed_node) or None
//
// Link enum is an Option, so the link is either pointing to another
// node when the Option's variant is Some or it has a None variant
// that represents a null i.e. the end of list.
//
// Important: Link is the thing we manipulate (pop, push, append, etc.)


// empty list (1 link):
//
//      head: Link
// List ----------> ∅
//        None
//
// List.head = None


// list with 1 node (2 links):
//
//      head: Link         next: Link
// List ----------> [Node] ----------> ∅
//        Some(_)            None
//
// List.head = Some(Box::Node).next = None


// list with 2 nodes (3 links):
//
//      head: Link         next: Link         next: Link
// List ----------> [Node] ----------> [Node] ----------> ∅
//        Some(_)            Some(_)              None

// List.head = Some(Box::Node).next = Some(Box::Node).next = None