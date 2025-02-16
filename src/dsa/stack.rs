use crate::utils;
use std::fmt::Display;
use std::mem;
use std::ops::Drop;

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node { val, next: None }
    }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        println!("Dropping node with address {:p}", &self.val);
    }
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> LinkedList<T>
where
    T: Display + Clone,
{
    fn new() -> Self {
        LinkedList {
            head: None,
            length: 0,
        }
    }

    fn peek_head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }

    fn peek_head_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.val)
    }

    fn push_front(&mut self, val: T) {
        let mut new_node = Box::new(Node::new(val));
        new_node.next = self.head.take();
        self.head = Some(new_node);
        self.length += 1;
    }

    fn pop_head(&mut self) -> Option<T> {
        match self.head.take() {
            Some(mut node) => {
                self.head = node.next.take();
                self.length -= 1;
                Some(node.val.clone())
                // Dropping happens here. `node` is dropped when it goes out of scope.
            }
            None => None,
        }
    }

    fn reverse(&mut self) {
        if self.head.is_none() {
            return;
        }
        let mut curr = self.head.take();
        let mut prev: Option<Box<Node<T>>> = None;
        while let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            curr = next;
        }

        self.head = prev;
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.head.is_some() {
            self.head = self.head.take().and_then(|mut node| {
                self.length -= 1;
                node.next.take()
            });
        }
    }
}

struct Stack<T> {
    list: LinkedList<T>,
}

impl<T> Stack<T>
where
    T: Clone + Display,
{
    fn new() -> Self {
        Stack {
            list: LinkedList::new(),
        }
    }

    /// Creates a new stack from a vector. Top of the stack is the last element of the vector.
    fn from_vec(vec: Vec<T>) -> Self {
        let mut list = LinkedList::new();
        for val in vec {
            list.push_front(val);
        }
        Stack { list }
    }

    /// Converts the stack to a vector. Top of the stack is the last element of the vector.
    fn to_vec(&self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut current = &self.list.head;
        while let Some(node) = current {
            vec.push(node.val.clone());
            current = &node.next;
        }
        vec
    }

    fn push(&mut self, val: T) {
        self.list.push_front(val);
    }

    fn pop(&mut self) -> Option<T> {
        self.list.pop_head()
    }

    fn peek(&self) -> Option<&T> {
        self.list.peek_head()
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.list.peek_head_mut()
    }

    fn len(&self) -> usize {
        self.list.length
    }

    fn is_empty(&self) -> bool {
        println!("Stack length: {}", self.list.length);
        self.list.length == 0
    }

    fn print(&self) {
        if self.is_empty() {
            println!("(empty)");
            println!("-------");
            return;
        }

        let mut current = &self.list.head;
        let mut max_width = 0;

        while let Some(node) = current {
            let value_str = format!("{}", node.val);
            max_width = max_width.max(value_str.len());
            current = &node.next;
        }

        let elements = self.to_vec();
        for elem in elements.iter().rev() {
            println!("| {:^max_width$} |", elem, max_width = max_width);
        }

        // Print bottom border with dynamic width
        println!("{}", "-".repeat(max_width + 4));
    }

    fn reverse(&mut self) {
        self.list.reverse();
    }
}

pub fn launch() {
    let mut stack = Stack::new();

    loop {
        utils::clear_screen();
        stack.print();
        println!("Data Structure Operations");
        println!("1. Push onto Stack / Initialize New Stack");
        println!("2. Pop from Stack");
        println!("3. Peek at Stack");
        println!("4. Check if Stack is empty");
        println!("5. Get Stack length");
        println!("6. Reverse Stack");
        println!("7. Convert Stack to Vector");
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
                println!("- a value to push to the current stack");
                println!("- or multiple values separated by commas (from bottom to top) to initialize a new stack");
                let input = utils::read_input();
                let values: Vec<&str> = input.split(',').map(|s| s.trim()).collect();
                if values.len() == 1 {
                    stack.push(values[0].to_string());
                } else {
                    stack = Stack::from_vec(values.iter().map(|s| s.to_string()).collect());
                }
            }
            2 => match stack.pop() {
                Some(value) => println!("Popped value: {}", value),
                None => println!("Stack is empty."),
            },
            3 => match stack.peek() {
                Some(peek) => {
                    println!("Top value: {}", peek);
                    println!("Do you want to change the value? (y/n)");
                    let input = utils::read_input();
                    if input.trim().to_lowercase() != "y" {
                        continue;
                    }
                    println!("Change value to:");
                    let new_value = utils::read_input();
                    if new_value.is_empty() {
                        println!("No value entered.");
                    } else if let Some(peek_mut) = stack.peek_mut() {
                        peek_mut.replace_range(.., &new_value);
                    }
                }
                None => println!("Stack is empty."),
            },
            4 => {
                if stack.is_empty() {
                    println!("Stack is empty.");
                } else {
                    println!("Stack is not empty.");
                }
            }
            5 => {
                println!("Stack length: {}", stack.len());
            }
            6 => {
                stack.reverse();
                println!("Stack reversed.");
            }
            7 => {
                let vec = stack.to_vec();
                println!("Stack as vector: {:?}", vec);
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
    println!("Dropping stack");
    mem::drop(stack);
    println!("Press Enter to confirm...");
    utils::read_input();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_head(), Some(3));
        assert_eq!(list.pop_head(), Some(2));
        assert_eq!(list.pop_head(), Some(1));
        assert_eq!(list.pop_head(), None);
    }

    #[test]
    fn test_linked_list_reverse() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.reverse();
        assert_eq!(list.pop_head(), Some(1));
        assert_eq!(list.pop_head(), Some(2));
        assert_eq!(list.pop_head(), Some(3));
        assert_eq!(list.pop_head(), None);
    }

    #[test]
    fn peek() {
        let mut list = LinkedList::new();
        assert_eq!(list.peek_head(), None);
        assert_eq!(list.peek_head_mut(), None);
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.peek_head(), Some(&3));
        assert_eq!(list.peek_head_mut(), Some(&mut 3));
        if let Some(value) = list.peek_head_mut() {
            *value = 42;
        }
        assert_eq!(list.peek_head(), Some(&42));
        assert_eq!(list.pop_head(), Some(42));
    }
}
