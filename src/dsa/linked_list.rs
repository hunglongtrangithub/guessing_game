use crate::utils;
use std::boxed::Box;

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node { val, next: None }
    }
}

pub fn launch() {
    loop {
        utils::clear_screen();
        println!("Data structures and algorithms");
        let node = Node::new(10);
        println!("Node value: {}", node.val);
        println!("Continue? (y/n)");
        let continue_or_not = utils::read_input();
        if continue_or_not.trim() != "y" {
            break;
        }
    }
}
