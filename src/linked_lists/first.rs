// A Bad Singly-Linked List

// pub says we want people outside this module to be able to use List
pub enum BadList {
    Empty,
    Elem(i32, Box<BadList>),
}
//
// BadList is bad because it would look like this:
// [Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)
// where the first element is on the stack
// we get junk because we alocate the biggest amount of space
// Also, splitting the list would bring a heap node onto the stack
//


pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}
