use std::cell::RefCell;
use std::rc::{ Rc, Weak };

type Strong<T> = Rc<RefCell<Node<T>>>;
type WeakPtr<T> = Weak<RefCell<Node<T>>>;

struct List<T> {
    head: Option<Strong<T>>,
    tail: Option<Strong<T>>,
}

struct Node<T> {
    element: T,
    prev: Option<WeakPtr<T>>, // non-owning
    next: Option<Strong<T>>, // owning
}

impl<T> Node<T> {
    fn new(element: T) -> Strong<T> {
        Rc::new(
            RefCell::new(Node {
                element,
                prev: None,
                next: None,
            })
        )
    }
}

impl<T: std::fmt::Display> List<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, element: T) {
        let new_head = Node::new(element);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_head));
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    fn push_back(&mut self, element: T) {
        let new_tail = Node::new(element);

        match self.tail.take() {
            Some(old_tail) => {
                new_tail.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                old_tail.borrow_mut().next = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    fn remove_front(&mut self) {
        match self.head.take() {
            Some(old_head) => {
                match old_head.borrow_mut().next.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().prev = None;
                        self.head = Some(new_head);
                    }
                    None => {
                        self.tail = None;
                        println!("List is now empty after removing the front element.");
                    }
                }
            }
            None => println!("List is empty, nothing to remove."),
        }
    }

    fn remove_back(&mut self) {
        match self.tail.take() {
            Some(old_tail) => {
                match old_tail.borrow_mut().prev.take() {
                    Some(prev_weak) => {
                        if let Some(prev) = prev_weak.upgrade() {
                            prev.borrow_mut().next = None;
                            self.tail = Some(prev);
                        }
                    }
                    None => {
                        self.head = None;
                        println!("List is now empty after removing the back element.");
                    }
                }
            }
            None => println!("List is empty, nothing to remove."),
        }
    }

    fn print(&self) {
        if self.head.is_none() {
            println!("[]");
            return;
        }

        let mut traversal = self.head.clone();
        while let Some(node) = traversal {
            print!("{} ", node.borrow().element);
            traversal = node.borrow().next.clone();
        }
        println!();
    }
}

fn main() {
    let mut doubly_ll = List::new();

    doubly_ll.remove_front();
    doubly_ll.push_front(10);
    doubly_ll.print();

    doubly_ll.push_back(20);
    doubly_ll.print();

    doubly_ll.push_front(5);
    doubly_ll.print();

    doubly_ll.remove_back();
    doubly_ll.print();

    doubly_ll.remove_front();
    doubly_ll.print();
}
