use crate::utils;
use std::ops::Drop;

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node { val, next: None }
    }
}

impl Drop for Node {
    /// To demonstrate how nodes are dropped
    fn drop(&mut self) {
        println!("Dropping Node with value {}", self.val);
    }
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push_front(&mut self, val: i32) {
        let mut new_node = Box::new(Node::new(val));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    fn pop_front(&mut self) -> Option<i32> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            node.val
        })
    }

    fn drop(mut self) {
        while self.head.is_some() {
            let val = self.pop_front();
            println!("Popped node with value {:?}", val);
        }
    }

    fn reverse(&mut self) {
        if self.head.is_none() {
            return;
        }
        let mut curr = self.head.take();
        let mut prev: Option<Box<Node>> = None;
        while let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            curr = next;
        }

        self.head = prev;
    }

    fn from_vec(vec: Vec<i32>) -> Self {
        let mut list = LinkedList::new();
        for val in vec.iter().rev() {
            list.push_front(*val)
        }
        list
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut vec = Vec::new();
        let mut current = &self.head;
        while let Some(node) = current {
            vec.push(node.val);
            current = &node.next;
        }
        vec
    }

    fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.val);
            current = &node.next;
        }
        println!("None");
    }
}

pub fn launch() {
    loop {
        utils::clear_screen();
        println!("Linked List");

        println!("Write a list of numbers separated by spaces");
        let input = utils::read_input();
        let vec: Vec<i32> = input
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        {
            let mut list = LinkedList::from_vec(vec);
            list.print();

            list.reverse();
            println!("Reversed list:");
            list.print();

            println!("To vector:");
            println!("{:?}", list.to_vec());

            println!("Drop the linked list using mem::drop (y) or drop method (n)?");
            match utils::read_input().trim() {
                "n" => {
                    println!("Using drop method:");
                    list.drop();
                }
                _ => {
                    println!("Using mem::drop:");
                }
            }
        }
        println!("You should see the nodes being dropped before this message");
        println!("Continue? (y/n)");
        if utils::read_input().trim() != "y" {
            break;
        }
    }
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
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_linked_list_reverse() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.reverse();
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_linked_list_from_vec() {
        let list = LinkedList::from_vec(vec![1, 2, 3]);
        assert_eq!(list.to_vec(), vec![1, 2, 3]);
    }
}
