use crate::utils;
use std::cell::RefCell;
use std::fmt::Display;
use std::mem;
use std::rc::Rc;

struct DoublyNode<T> {
    val: T,
    next: Option<Rc<RefCell<DoublyNode<T>>>>,
    prev: Option<Rc<RefCell<DoublyNode<T>>>>,
}

impl<T> DoublyNode<T> {
    fn new(val: T) -> Self {
        DoublyNode {
            val,
            next: None,
            prev: None,
        }
    }
}

impl<T> Drop for DoublyNode<T> {
    fn drop(&mut self) {
        println!("Dropping node with address {:p}", &self.val);
    }
}

struct DoublyLinkedList<T> {
    header: Rc<RefCell<DoublyNode<T>>>,  // Sentinel node at start
    trailer: Rc<RefCell<DoublyNode<T>>>, // Sentinel node at end
    length: usize,
}

impl<T> DoublyLinkedList<T>
where
    T: Display + Clone + Default,
{
    fn new() -> Self {
        let header = Rc::new(RefCell::new(DoublyNode::new(T::default())));
        let trailer = Rc::new(RefCell::new(DoublyNode::new(T::default())));

        // Link the sentinels
        header.borrow_mut().next = Some(Rc::clone(&trailer));
        trailer.borrow_mut().prev = Some(Rc::clone(&header));

        DoublyLinkedList {
            header,
            trailer,
            length: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn len(&self) -> usize {
        self.length
    }

    fn peek_head(&self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.header
                .borrow()
                .next
                .as_ref()
                .map(|node| node.borrow().val.clone())
        }
    }

    fn peek_tail(&self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.trailer
                .borrow()
                .prev
                .as_ref()
                .map(|node| node.borrow().val.clone())
        }
    }

    fn push_head(&mut self, val: T) {
        let new_head = Rc::new(RefCell::new(DoublyNode::new(val)));

        let old_head = self.header.borrow().next.as_ref().unwrap().clone();

        new_head.borrow_mut().next = Some(Rc::clone(&old_head));
        new_head.borrow_mut().prev = Some(Rc::clone(&self.header));

        old_head.borrow_mut().prev = Some(Rc::clone(&new_head));
        self.header.borrow_mut().next = Some(new_head);

        self.length += 1;
    }

    fn push_tail(&mut self, val: T) {
        let new_tail = Rc::new(RefCell::new(DoublyNode::new(val)));

        let old_tail = self.trailer.borrow().prev.as_ref().unwrap().clone();

        new_tail.borrow_mut().prev = Some(Rc::clone(&old_tail));
        new_tail.borrow_mut().next = Some(Rc::clone(&self.trailer));

        old_tail.borrow_mut().next = Some(Rc::clone(&new_tail));
        self.trailer.borrow_mut().prev = Some(new_tail);

        self.length += 1;
    }

    fn pop_head(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let old_head = self.header.borrow().next.as_ref().unwrap().clone();
        let new_head = old_head.borrow().next.as_ref().unwrap().clone();

        self.header.borrow_mut().next = Some(Rc::clone(&new_head));
        new_head.borrow_mut().prev = Some(Rc::clone(&self.header));

        self.length -= 1;
        Some(old_head.clone().borrow().val.clone())
    }

    fn pop_tail(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let old_tail = self.trailer.borrow().prev.as_ref().unwrap().clone();
        let new_tail = old_tail.borrow().prev.as_ref().unwrap().clone();

        self.trailer.borrow_mut().prev = Some(new_tail.clone());
        new_tail.borrow_mut().next = Some(Rc::clone(&self.trailer));

        self.length -= 1;
        Some(old_tail.clone().borrow().val.clone())
    }

    fn print(&self) {
        let mut curr = self.header.borrow().next.as_ref().unwrap().clone();
        print!("(Head) <-> ");
        while !Rc::ptr_eq(&curr, &self.trailer) {
            print!("{} <-> ", curr.borrow().val);
            curr = curr.clone().borrow().next.as_ref().unwrap().clone();
        }
        println!("(Tail)");
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        let mut curr = Some(self.header.clone());
        while let Some(node) = curr {
            let next = node.borrow().next.clone();
            node.borrow_mut().next = None;
            node.borrow_mut().prev = None;
            curr = next;
        }
    }
}

struct Deque<T> {
    list: DoublyLinkedList<T>,
}

impl<T> Deque<T>
where
    T: Display + Clone + Default,
{
    fn new() -> Self {
        Deque {
            list: DoublyLinkedList::new(),
        }
    }

    fn empty(&self) -> bool {
        self.list.is_empty()
    }

    fn len(&self) -> usize {
        self.list.len()
    }

    fn from_vec(vec: Vec<T>) -> Self {
        let mut list = DoublyLinkedList::new();
        for val in vec {
            list.push_tail(val);
        }
        Deque { list }
    }

    fn to_vec(&self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut curr = self.list.header.borrow().next.as_ref().unwrap().clone();
        while !Rc::ptr_eq(&curr, &self.list.trailer) {
            vec.push(curr.borrow().val.clone());
            curr = curr.clone().borrow().next.as_ref().unwrap().clone();
        }
        vec
    }
    fn peek_front(&self) -> Option<T> {
        self.list.peek_head()
    }

    fn peek_back(&self) -> Option<T> {
        self.list.peek_tail()
    }

    fn push_front(&mut self, val: T) {
        self.list.push_head(val);
    }

    fn push_back(&mut self, val: T) {
        self.list.push_tail(val);
    }

    fn pop_front(&mut self) -> Option<T> {
        self.list.pop_head()
    }

    fn pop_back(&mut self) -> Option<T> {
        self.list.pop_tail()
    }

    fn print(&self) {
        self.list.print();
    }
}

pub fn launch() {
    let mut deque = Deque::new();

    loop {
        utils::clear_screen();
        deque.print();
        println!("Deque Operations");
        println!("1. Push Front / Initialize New Deque");
        println!("2. Push Back");
        println!("3. Pop Front");
        println!("4. Pop Back");
        println!("5. Peek Front");
        println!("6. Peek Back");
        println!("7. Check if Empty");
        println!("8. Get Length");
        println!("9. Convert Deque to Vector");
        println!("0. Exit");

        let choice = utils::read_input();
        let choice = match choice.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid selection. Please enter a valid number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter:");
                println!("- a single value to push to front");
                println!("- or multiple values separated by commas from front to back to initialize a new deque");
                let input = utils::read_input();
                let values: Vec<&str> = input.split(',').map(|s| s.trim()).collect();

                if values.len() == 1 {
                    deque.push_front(values[0].to_string());
                } else {
                    deque = Deque::from_vec(values.iter().map(|s| s.to_string()).collect());
                }
            }
            2 => {
                println!("Enter a single value to push to back:");
                let input = utils::read_input();
                deque.push_back(input);
            }
            3 => match deque.pop_front() {
                Some(value) => println!("Popped from front: {}", value),
                None => println!("Deque is empty."),
            },
            4 => match deque.pop_back() {
                Some(value) => println!("Popped from back: {}", value),
                None => println!("Deque is empty."),
            },
            5 => match deque.peek_front() {
                Some(value) => println!("Front of deque: {}", value),
                None => println!("Deque is empty."),
            },
            6 => match deque.peek_back() {
                Some(value) => println!("Back of deque: {}", value),
                None => println!("Deque is empty."),
            },
            7 => {
                if deque.empty() {
                    println!("Deque is empty.");
                } else {
                    println!("Deque is not empty.");
                }
            }
            8 => {
                println!("Deque length: {}", deque.len());
            }
            9 => {
                let vec = deque.to_vec();
                println!("Deque as vector: {:?}", vec);
            }
            0 => {
                break;
            }
            _ => {
                println!("Invalid selection. Please enter a valid number.");
            }
        }
        println!("Press Enter to continue...");
        utils::read_input();
    }
    println!("Dropping deque...");
    mem::drop(deque);
    println!("Press Enter to confirm...");
    utils::read_input();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deque_basic() {
        let mut deque = Deque::new();
        assert!(deque.empty());

        deque.push_front("1".to_string());
        assert_eq!(deque.len(), 1);
        assert_eq!(deque.peek_front(), Some("1".to_string()));
        assert_eq!(deque.peek_back(), Some("1".to_string()));
    }

    #[test]
    fn test_deque_push_pop() {
        let mut deque = Deque::new();

        deque.push_front("1".to_string());
        deque.push_back("2".to_string());
        deque.push_front("0".to_string());

        assert_eq!(deque.pop_front(), Some("0".to_string()));
        assert_eq!(deque.pop_back(), Some("2".to_string()));
        assert_eq!(deque.pop_front(), Some("1".to_string()));
        assert!(deque.empty());
    }

    #[test]
    fn test_deque_mixed_operations() {
        let mut deque = Deque::new();

        deque.push_back("1".to_string());
        deque.push_front("0".to_string());
        assert_eq!(deque.peek_front(), Some("0".to_string()));
        assert_eq!(deque.peek_back(), Some("1".to_string()));

        assert_eq!(deque.pop_back(), Some("1".to_string()));
        assert_eq!(deque.pop_back(), Some("0".to_string()));
        assert!(deque.empty());
    }
}
