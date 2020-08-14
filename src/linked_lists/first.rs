// A Bad Singly-Linked List

use std::mem;

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

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    // push onto the head of a list
    //
    // we have a mutable ref to self
    // the tricky part of replacing makes sense because we have to have an empty tail
    pub fn push(&mut self, value: i32)  {
        let node = Node {
            elem: value,
            // next: self.head, // wont work, moving a borrowed value
            next: mem::replace(&mut self.head, Link::Empty) // the old indy swap
        };

        // rust will be mad at us unless we give something back
        self.head = Link::More(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

    fn pop_node(&mut self) -> Link {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => Link::Empty,
            Link::More(mut node) => {
                self.head = mem::replace(&mut node.next, Link::Empty);
                Link::More(node)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(_) = self.pop_node() { }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));

        assert_eq!(list.pop(), None);
    }
}
