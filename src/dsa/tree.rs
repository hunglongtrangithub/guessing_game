use crate::utils;
use std::mem;

struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        println!("Dropping node at address: {:p}", &self);
    }
}

struct BinarySearchTree {
    root: Option<Box<Node<i32>>>,
}

impl Drop for BinarySearchTree {
    fn drop(&mut self) {
        let mut stack = Vec::new();
        if let Some(root) = self.root.take() {
            stack.push(root);
        }
        while let Some(mut node) = stack.pop() {
            if let Some(right) = node.right.take() {
                stack.push(right);
            }
            if let Some(left) = node.left.take() {
                stack.push(left);
            }
            // Node is dropped here
        }
    }
}

impl BinarySearchTree {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn insert(&mut self, value: i32) {
        let new_node = Box::new(Node::new(value));
        match self.root.as_mut() {
            Some(mut current_node) => loop {
                if value < current_node.value {
                    // if value is less than current node, go left
                    match current_node.left {
                        Some(ref mut left) => current_node = left,
                        None => {
                            current_node.left = Some(new_node);
                            return;
                        }
                    }
                } else {
                    // if value is greater than current node, go right
                    match current_node.right {
                        Some(ref mut right) => current_node = right,
                        None => {
                            current_node.right = Some(new_node);
                            return;
                        }
                    }
                }
            },
            None => self.root = Some(new_node),
        }
    }

    fn search(&self, value: i32) -> bool {
        let mut current_node = self.root.as_ref();
        while let Some(node) = current_node {
            match node.value.cmp(&value) {
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Less => current_node = node.right.as_ref(),
                std::cmp::Ordering::Greater => current_node = node.left.as_ref(),
            }
        }
        false
    }

    fn delete(&mut self, value: i32) -> bool {
        let (new_root, found) = delete_node(self.root.take(), value);
        self.root = new_root;
        return found;
        fn delete_node(node: Option<Box<Node<i32>>>, value: i32) -> (Option<Box<Node<i32>>>, bool) {
            match node {
                Some(mut node) => match node.value.cmp(&value) {
                    std::cmp::Ordering::Equal => match (node.left.take(), node.right.take()) {
                        // leaf node -> remove it
                        (None, None) => (None, true),
                        // one child -> replace node with child
                        (Some(left), None) => (Some(left), true),
                        (None, Some(right)) => (Some(right), true),
                        // two children
                        (Some(left), Some(right)) => {
                            fn remove_min(
                                mut node: Box<Node<i32>>,
                            ) -> (i32, Option<Box<Node<i32>>>) {
                                if let Some(left) = node.left.take() {
                                    // Recursively find the minimum value in the left subtree; replace the left child with the new subtree.
                                    let (min_value, new_left) = remove_min(left);
                                    node.left = new_left;
                                    (min_value, Some(node))
                                } else {
                                    // This node is the minimum; return its value and replace it with its right child
                                    (node.value, node.right.take())
                                }
                            }
                            let (min_value, new_right) = remove_min(right);
                            node.value = min_value;
                            node.right = new_right;
                            node.left = Some(left);
                            (Some(node), true)
                        }
                    },
                    std::cmp::Ordering::Less => {
                        let (new_right, found) = delete_node(node.right.take(), value);
                        node.right = new_right;
                        (Some(node), found)
                    }
                    std::cmp::Ordering::Greater => {
                        let (new_left, found) = delete_node(node.left.take(), value);
                        node.left = new_left;
                        (Some(node), found)
                    }
                },
                None => (None, false),
            }
        }
    }

    fn in_order_traversal(&self) {
        fn traverse(node: &Option<Box<Node<i32>>>) {
            if let Some(node) = node {
                traverse(&node.left);
                print!("{} ", node.value);
                traverse(&node.right);
            }
        }
        traverse(&self.root);
    }
}

pub fn launch() {
    let mut bst = BinarySearchTree::new();

    loop {
        utils::clear_screen();
        println!("Binary Search Tree Operations");
        println!("1. Insert a value / Make a new tree");
        println!("2. Search for a value");
        println!("3. Delete a value");
        println!("4. Display tree (In-order Traversal)");
        println!("5. Check if tree is empty");
        println!("6. Clear tree");
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
                println!("Enter a value to insert, or multiple values separated by spaces to make a new tree:");
                let input = utils::read_input();
                let values: Vec<i32> = input
                    .split_whitespace()
                    .filter_map(|s| s.parse::<i32>().ok())
                    .collect();
                if values.is_empty() {
                    println!("No valid values entered.");
                } else {
                    bst.root = None;
                    for value in values {
                        bst.insert(value);
                    }
                    println!("Inserted values into the BST.");
                }
            }
            2 => {
                println!("Enter a value to search:");
                let input = utils::read_input();
                if let Ok(value) = input.trim().parse::<i32>() {
                    if bst.search(value) {
                        println!("Value {} found in the BST.", value);
                    } else {
                        println!("Value {} not found in the BST.", value);
                    }
                } else {
                    println!("Invalid input. Please enter a valid integer.");
                }
            }
            3 => {
                println!("Enter a value to delete:");
                let input = utils::read_input();
                if let Ok(value) = input.trim().parse::<i32>() {
                    if bst.delete(value) {
                        println!("Value {} deleted from the BST.", value);
                    } else {
                        println!("Value {} not found in the BST.", value);
                    }
                } else {
                    println!("Invalid input. Please enter a valid integer.");
                }
            }
            4 => {
                println!("In-order traversal of BST:");
                bst.in_order_traversal();
            }
            5 => {
                if bst.is_empty() {
                    println!("The BST is empty.");
                } else {
                    println!("The BST is not empty.");
                }
            }
            6 => {
                bst.root = None;
                println!("The BST has been cleared.");
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

    println!("Dropping BST...");
    mem::drop(bst);
    println!("Press Enter to confirm...");
    utils::read_input();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut bst = BinarySearchTree::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        bst.insert(6);
        bst.insert(8);
        assert!(bst.search(5));
        assert!(bst.search(3));
        assert!(bst.search(7));
        assert!(bst.search(2));
        assert!(bst.search(4));
        assert!(bst.search(6));
        assert!(bst.search(8));
        assert!(!bst.search(1));
        assert!(!bst.search(9));
    }

    #[test]
    fn test_search() {
        let mut bst = BinarySearchTree::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        bst.insert(6);
        bst.insert(8);
        assert!(bst.search(5));
        assert!(bst.search(3));
        assert!(bst.search(7));
        assert!(bst.search(2));
        assert!(bst.search(4));
        assert!(bst.search(6));
        assert!(bst.search(8));
        assert!(!bst.search(1));
        assert!(!bst.search(9));
    }

    #[test]
    fn test_delete() {
        let mut bst = BinarySearchTree::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        bst.insert(6);
        bst.insert(8);

        // Delete existing values
        assert!(bst.delete(5)); // Root node with two children
        assert!(!bst.search(5));
        assert!(bst.search(3));
        assert!(bst.search(7));

        assert!(bst.delete(3)); // Node with two children
        assert!(!bst.search(3));
        assert!(bst.search(2));
        assert!(bst.search(4));

        assert!(bst.delete(7)); // Node with two children
        assert!(!bst.search(7));
        assert!(bst.search(6));
        assert!(bst.search(8));

        assert!(bst.delete(2)); // Leaf node
        assert!(!bst.search(2));

        assert!(bst.delete(4)); // Leaf node
        assert!(!bst.search(4));

        assert!(bst.delete(6)); // Node with one child
        assert!(!bst.search(6));

        assert!(bst.delete(8)); // Last node
        assert!(!bst.search(8));

        // Empty tree
        assert!(bst.is_empty());

        // Try to delete non-existent values
        assert!(!bst.delete(10)); // Value never existed
        assert!(!bst.delete(5)); // Value existed but was deleted

        // Insert a new value and ensure we can delete it
        bst.insert(15);
        assert!(bst.delete(15));
        assert!(!bst.search(15));
        assert!(bst.is_empty());
    }
}
