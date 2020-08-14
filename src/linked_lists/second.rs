// An Ok Singly-Linked List

use std::mem;


pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, value: i32)  {
        let node = Node {
            elem: value,
            next: self.head.take()
        };

        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    fn pop_node(&mut self) -> Link {
        self.head.take().map(|node| {
            self.head = node.next.take();
            node
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let cur_link = self.head.take();

        while let Some(_) = self.pop_node() { }
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
