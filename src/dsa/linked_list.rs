use crate::utils;

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node { val, next: None }
    }
}

struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push_head(&mut self, val: i32) {
        let mut new_node = Box::new(Node::new(val));
        new_node.next = self.head.take();
        self.head = Some(new_node);
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
            list.push_head(*val)
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
            .map(|s| s.parse().expect("parse error"))
            .collect();
        let mut list = LinkedList::from_vec(vec);
        list.print();
        list.reverse();
        println!("Reversed list:");
        list.print();
        println!("To vector:");
        println!("{:?}", list.to_vec());
        println!("Continue? (y/n)");
        let continue_or_not = utils::read_input();
        if continue_or_not.trim() != "y" {
            break;
        }
    }
}
