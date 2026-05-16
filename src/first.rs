// pub declares something as public
// we have this structure to publically expose List and keep internals private
pub struct List {
    head: Link,
}

impl List {
    // This is a non-static function because it does not take in 
    // self (or a mutable or non-mutable reference to self) as a parameter
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            // Steal a value from a borrow by replacing it with another variable
            next: std::mem::replace(&mut self.head, Link::Empty),
        });

        // Set self.head to the value of the new node
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // Use pattern-matching over enums when possible
        // Match over a mutable reference because we want to change head
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                // Move the exisitng Box<Node> out of the self.head variant and brings it to node
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

/* 
    An enum in rust is a tagged union, and its size is
    the size of the tag/discriminant, the largest element possible in the union, 
    and any necessary padding bits the size of the tag can be 
    optimized away with the Niche Value Optimization, aka the Null Pointer Optimziation
    (NPO), where the compiler recognzies that there are patterns that are possible but 
    logically invalid, so it uses the null bit to represent that variant. 
    This is gauranteed for Option<T> when T is a reference or mutable reference, a 
    function pointer, a Boxed type, Non-zero integers (can't use 0 if it's a valid state),
    and smart pointers. 
*/
enum Link {
    Empty,
    /*
        A Box is used to limit the size of the List
        Box<T> allocates the exact amount of heap memory for type
        T at *runtime* by using the size_of::<T>() method. It keeps
        the data on the heap while having a pointer on the stack.
        The size of the enum is thus known at compile-time.
    */
    More(Box<Node>),
}

// I think that generics programming would specific T in this Node struct
// but that's outside the scope of this tutorial
struct Node {
    elem: i32,
    next: Link,
}


// A type has a destructor if it implements a trait (interface) Drop
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

// mod creates a new file inline
// it is idiomatic that you write tests next to the functions you define
#[cfg(test)] // only compile when you're running tests; silences unused import errors
mod test {
    use super::List; // need to do this because we made the new module
    #[test]  // marks it as test
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