use std::mem;

/* List a = Empty | Elem a (List a) */

/*
 * List is here to hold the entire structure in order to have a nice pointer.
 * A list consists of a link, which is explained later.
 * Reasoning behind this is that any given sub list can be cast as a List,
 * which is exactly the inked list behavior we want
 */
pub struct List {
    head: Link,
}

/*
 *   Null Pointer Optimization
 * Typically enums need to save space for their tag/variant as
 * well as data they hold.
 *
 * When an enum has the following shape, the Empty variant is
 * represented as all 0's. This means you can have either the
 * data(assumed to be More), or all 0's(assumed to be Empty)
 *
 * This is one node in the list, or a link in the chain.
 * It has two states, empty or not empty. More is just a
 * nice pointer to some heap memory that is the size of a node.
 */
enum Link {
    Empty,
    More(Box<Node>),
}

/*
 * Node holds the nice ordered pair of a value and a pointer.
 * It is not public because it is not supposed to be shared.
 * We only refer to it above in a Box, so that fixes the recursion
 * memory layout issue.
 *
 * The only difference between this and a trivial implementation is
 * the list struct at the top, which makes our list much nicer to work
 * with
 */
struct Node {
    elem: i32,
    next: Link,
}


impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    // we are mutating our list
    pub fn push(&mut self, elem: i32) {
        // start by initializing a new node
        // by Link definition, it needs to be in a Box
        let new_node = Box::new(Node {
            elem: elem,
            // head is a Link.
            // next needs to own its element.
            // reassigning self.head will move it( head is RHV ).
            // self is borrowed above,
            // so we have to give it back when were done.
            // moving/taking it is not how you borrow things nicely.
            // replace will replace p1 with p2, return p1.
            // this nicely trades data so we can have borrowed value.
            // empty is a placeholder to get around the borrow
            next: mem::replace(&mut self.head, Link::Empty),
        });

        // we need to tack this new node onto the end of the list.
        // since we are borrowing, we can totally change the value
        // this is because we're mutable and can use it as LHV
        // overwriting the empty from above
        self.head = Link::More(new_node);
    }

    // this time we have to return an option
    // could be empty, could have data
    pub fn pop(&mut self) -> Option<i32> {
        // pattern matching wants to move the value into the
        // branches below.
        // we have to use a mutable borrow
        // replace will replace p1 with p2, return p1.
        // head goes mutably into the match
        // empty is a placeholder
        match mem::replace(&mut self.head, Link::Empty) {
            // if its empty, return None
            Link::Empty => None,
            // else pull the node out and move node.next into the head
            // essentially closing a gap in chain
            // overwriting the empty placeholder
            // return the option wrapped element
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
        // trade the value with empty, which can drop itself
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        /*
        // `while let` == "do this thing until this pattern doesn't match"
        // we have to do this until our list ends with Empty
        while let Link::More(mut boxed_node) = cur_link {
            // again trade that value with empty and hold onto it with cur_link
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
        */
        while let Link::More(_) = self.pop_node() { }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
