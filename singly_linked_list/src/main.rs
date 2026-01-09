#[derive(Debug)]
struct SinglyLinkedList<T: std::fmt::Debug + std::marker::Copy> {
    head: Pointer<T>,
}

#[derive(Debug)]
struct Node<T: std::fmt::Debug + std::marker::Copy> {
    element: T,
    next: Pointer<T>,
}

type Pointer<T> = Option<Box<Node<T>>>;

impl<T: std::fmt::Debug + std::marker::Copy> SinglyLinkedList<T> {
    fn create_empty_list() -> SinglyLinkedList<T> {
        SinglyLinkedList { head: None }
    }

    fn add(&mut self, element: T) {
        let previous_head = self.head.take();
        let new_head = Box::new(Node {
            element: element,
            next: previous_head,
        });
        self.head = Some(new_head);
    }

    fn remove(&mut self) -> Option<T> {
        let previous_head = self.head.take();
        match previous_head {
            Some(old_head) => {
                self.head = old_head.next;
                Some(old_head.element)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<T> {
        match &self.head {
            Some(node) => Some(node.element),
            None => None,
        }
    }

    fn print(&self) {
        let mut traversal = &self.head;
        while true {
            match traversal {
                Some(node) => {
                    print!("{:?} ", node.element);
                    traversal = &node.next;
                }
                None => {
                    break;
                }
            }
        }
    }
}

fn main() {
    // let list = SinglyLinkedList { head: None };

    // let list = SinglyLinkedList {
    //     head: Some(Box::new(Node {
    //         element: 10,
    //         next: Some(Box::new(Node {
    //             element: 20,
    //             next: None,
    //         })),
    //     })),
    // };
    // // println!("Head element: {}", list.head.unwrap().element);
    // //println!("List: {:?}", list.head.unwrap().next.unwrap().element);
    // println!("List: {:?}", list.head);

    let mut list = SinglyLinkedList::create_empty_list();
    list.add(10);
    list.add(20);
    list.add(30);
    list.add(40);
    println!("\nList: {:?}", list);

    list.remove();
    println!("\nList after removal: {:?}", list);

    println!("\nPeek element: {:?}", list.peek());

    print!("\nList elements: ");
    list.print();
}
