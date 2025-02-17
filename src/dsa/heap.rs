use crate::utils;
use std::fmt::Display;
use std::mem;

#[derive(Clone, Copy)]
enum HeapType {
    MinHeap,
    MaxHeap,
}

struct Heap<T> {
    data: Vec<T>,
    heap_type: HeapType,
}

impl<T> Heap<T>
where
    T: Ord + Clone + Display,
{
    fn new(heap_type: HeapType) -> Self {
        Heap {
            data: Vec::new(),
            heap_type,
        }
    }

    fn from_vec(vec: Vec<T>, heap_type: HeapType) -> Self {
        let mut heap = Heap {
            data: vec,
            heap_type,
        };
        let len = heap.data.len();
        for i in (0..len / 2).rev() {
            heap.heapify_down(i);
        }
        heap
    }

    fn to_vec(&self) -> Vec<T> {
        self.data.clone()
    }

    fn replace(&mut self, val: T) -> Option<T> {
        if self.data.is_empty() {
            self.data.push(val);
            return None;
        }
        let max = self.data[0].clone();
        self.data[0] = val;
        self.heapify_down(0);
        Some(max)
    }

    fn merge(&mut self, other: &mut Self) {
        self.data.append(&mut other.data);
        let len = self.data.len();
        for i in (0..len / 2).rev() {
            self.heapify_down(i);
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
        self.heapify_up(self.data.len() - 1);
    }

    fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let last_index = self.data.len() - 1;
        self.data.swap(0, last_index);
        let max = self.data.pop();
        self.heapify_down(0);
        max
    }

    fn compare(&self, a: &T, b: &T) -> bool {
        match self.heap_type {
            HeapType::MinHeap => a < b,
            HeapType::MaxHeap => a > b,
        }
    }
    fn heapify_up(&mut self, index: usize) {
        let mut index = index;
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.compare(&self.data[index], &self.data[parent]) {
                self.data.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    fn heapify_down(&mut self, index: usize) {
        let mut index = index;
        loop {
            let left = 2 * index + 1;
            let right = 2 * index + 2;
            let mut largest = index;
            if left < self.data.len() && self.compare(&self.data[left], &self.data[largest]) {
                largest = left;
            }
            if right < self.data.len() && self.compare(&self.data[right], &self.data[largest]) {
                largest = right;
            }
            if largest == index {
                break;
            }
            self.data.swap(index, largest);
            index = largest;
        }
    }

    fn print(&self) {
        // arrange data in a tree format
        let mut level = 0;
        let mut index = 0;
        while index < self.data.len() {
            for _ in 0..2usize.pow(level) {
                if index < self.data.len() {
                    print!("{} ", self.data[index]);
                    index += 1;
                }
            }
            println!();
            level += 1;
        }
    }
}
pub fn launch() {
    utils::clear_screen();
    println!("Please select the type of heap:");
    println!("1. Min Heap");
    println!("2. Max Heap");

    let heap_type = match utils::read_input().trim().parse::<u32>() {
        Ok(1) => HeapType::MinHeap,
        Ok(2) => HeapType::MaxHeap,
        _ => {
            println!("Invalid selection. Defaulting to Max Heap.");
            HeapType::MaxHeap
        }
    };

    let mut heap: Heap<i32> = Heap::new(heap_type);

    loop {
        utils::clear_screen();
        heap.print();
        println!("Heap Operations");
        println!("1. Push / Initialize New Heap");
        println!("2. Pop");
        println!("3. Peek");
        println!("4. Replace Top");
        println!("5. Merge with Another Heap");
        println!("6. Check if Empty");
        println!("7. Get Length");
        println!("8. Convert Heap to Vector");
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
                println!("- a single number to push");
                println!("- or multiple numbers separated by commas to initialize a new heap");
                let input = utils::read_input();
                let values: Vec<&str> = input.split(',').map(|s| s.trim()).collect();

                if values.len() == 1 {
                    if let Ok(value) = values[0].parse::<i32>() {
                        heap.push(value);
                        println!("Pushed: {}", value);
                    } else {
                        println!("Invalid input. Please enter a valid number.");
                    }
                } else {
                    let parsed_values: Vec<i32> = values
                        .iter()
                        .filter_map(|s| s.parse::<i32>().ok())
                        .collect();
                    heap = Heap::from_vec(parsed_values, heap_type);
                    println!("Created new heap from values");
                }
            }
            2 => match heap.pop() {
                Some(value) => println!("Popped: {}", value),
                None => println!("Heap is empty."),
            },
            3 => match heap.peek() {
                Some(value) => println!("Top of heap: {}", value),
                None => println!("Heap is empty."),
            },
            4 => {
                println!("Enter a value to replace the top element:");
                if let Ok(value) = utils::read_input().parse::<i32>() {
                    match heap.replace(value) {
                        Some(old_value) => println!("Replaced {} with {}", old_value, value),
                        None => println!("Heap was empty, pushed {} as first element", value),
                    }
                } else {
                    println!("Invalid input. Please enter a valid number.");
                }
            }
            5 => {
                println!("Enter values separated by commas for the second heap:");
                let input = utils::read_input();
                let values: Vec<i32> = input
                    .split(',')
                    .filter_map(|s| s.trim().parse::<i32>().ok())
                    .collect();
                let mut other_heap = Heap::from_vec(values, heap_type);
                heap.merge(&mut other_heap);
                println!("Heaps merged successfully");
            }
            6 => {
                if heap.is_empty() {
                    println!("Heap is empty.");
                } else {
                    println!("Heap is not empty.");
                }
            }
            7 => {
                println!("Heap length: {}", heap.len());
            }
            8 => {
                let vec = heap.to_vec();
                println!("Heap as vector: {:?}", vec);
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

    println!("Dropping heap...");
    mem::drop(heap);
    println!("Press Enter to confirm...");
    utils::read_input();
}

#[cfg(test)]
mod tests {
    use super::*;

    // Max Heap Tests
    mod max_heap_tests {
        use super::*;

        #[test]
        fn test_new_max_heap() {
            let heap: Heap<i32> = Heap::new(HeapType::MaxHeap);
            assert!(heap.is_empty());
            assert_eq!(heap.len(), 0);
        }

        #[test]
        fn test_push_pop_max_heap() {
            let mut heap = Heap::new(HeapType::MaxHeap);
            heap.push(5);
            heap.push(3);
            heap.push(7);
            heap.push(1);

            assert_eq!(heap.len(), 4);
            assert_eq!(heap.peek(), Some(&7));
            assert_eq!(heap.pop(), Some(7));
            assert_eq!(heap.pop(), Some(5));
            assert_eq!(heap.pop(), Some(3));
            assert_eq!(heap.pop(), Some(1));
            assert_eq!(heap.pop(), None);
        }

        #[test]
        fn test_from_vec_max_heap() {
            let vec = vec![1, 5, 3, 7, 2];
            let heap = Heap::from_vec(vec, HeapType::MaxHeap);
            assert_eq!(heap.peek(), Some(&7));

            let sorted = {
                let mut h = heap;
                let mut v = Vec::new();
                while let Some(x) = h.pop() {
                    v.push(x);
                }
                v
            };
            assert_eq!(sorted, vec![7, 5, 3, 2, 1]);
        }

        #[test]
        fn test_replace_max_heap() {
            let mut heap = Heap::from_vec(vec![5, 3, 7, 1], HeapType::MaxHeap);
            assert_eq!(heap.replace(4), Some(7));
            assert_eq!(heap.peek(), Some(&5));
        }

        #[test]
        fn test_merge_max_heap() {
            let mut heap1 = Heap::from_vec(vec![5, 3, 1], HeapType::MaxHeap);
            let mut heap2 = Heap::from_vec(vec![6, 4, 2], HeapType::MaxHeap);
            heap1.merge(&mut heap2);

            assert_eq!(heap1.len(), 6);
            assert_eq!(heap2.len(), 0);
            assert_eq!(heap1.peek(), Some(&6));
        }
    }

    // Min Heap Tests
    mod min_heap_tests {
        use super::*;

        #[test]
        fn test_new_min_heap() {
            let heap: Heap<i32> = Heap::new(HeapType::MinHeap);
            assert!(heap.is_empty());
            assert_eq!(heap.len(), 0);
        }

        #[test]
        fn test_push_pop_min_heap() {
            let mut heap = Heap::new(HeapType::MinHeap);
            heap.push(5);
            heap.push(3);
            heap.push(7);
            heap.push(1);

            assert_eq!(heap.len(), 4);
            assert_eq!(heap.peek(), Some(&1));
            assert_eq!(heap.pop(), Some(1));
            assert_eq!(heap.pop(), Some(3));
            assert_eq!(heap.pop(), Some(5));
            assert_eq!(heap.pop(), Some(7));
            assert_eq!(heap.pop(), None);
        }

        #[test]
        fn test_from_vec_min_heap() {
            let vec = vec![5, 3, 7, 1, 2];
            let heap = Heap::from_vec(vec, HeapType::MinHeap);
            assert_eq!(heap.peek(), Some(&1));

            let sorted = {
                let mut h = heap;
                let mut v = Vec::new();
                while let Some(x) = h.pop() {
                    v.push(x);
                }
                v
            };
            assert_eq!(sorted, vec![1, 2, 3, 5, 7]);
        }

        #[test]
        fn test_replace_min_heap() {
            let mut heap = Heap::from_vec(vec![5, 3, 7, 1], HeapType::MinHeap);
            assert_eq!(heap.replace(4), Some(1));
            assert_eq!(heap.peek(), Some(&3));
        }

        #[test]
        fn test_merge_min_heap() {
            let mut heap1 = Heap::from_vec(vec![5, 3, 1], HeapType::MinHeap);
            let mut heap2 = Heap::from_vec(vec![6, 4, 2], HeapType::MinHeap);
            heap1.merge(&mut heap2);

            assert_eq!(heap1.len(), 6);
            assert_eq!(heap2.len(), 0);
            assert_eq!(heap1.peek(), Some(&1));
        }
    }

    // Mixed Type Tests
    #[test]
    fn test_string_max_heap() {
        let mut heap = Heap::new(HeapType::MaxHeap);
        heap.push("apple".to_string());
        heap.push("zebra".to_string());
        heap.push("banana".to_string());

        assert_eq!(heap.pop(), Some("zebra".to_string()));
        assert_eq!(heap.pop(), Some("banana".to_string()));
        assert_eq!(heap.pop(), Some("apple".to_string()));
    }

    #[test]
    fn test_string_min_heap() {
        let mut heap = Heap::new(HeapType::MinHeap);
        heap.push("apple".to_string());
        heap.push("zebra".to_string());
        heap.push("banana".to_string());

        assert_eq!(heap.pop(), Some("apple".to_string()));
        assert_eq!(heap.pop(), Some("banana".to_string()));
        assert_eq!(heap.pop(), Some("zebra".to_string()));
    }

    #[test]
    fn test_empty_operations() {
        let mut max_heap: Heap<i32> = Heap::new(HeapType::MaxHeap);
        let mut min_heap: Heap<i32> = Heap::new(HeapType::MinHeap);

        assert_eq!(max_heap.pop(), None);
        assert_eq!(min_heap.pop(), None);
        assert_eq!(max_heap.peek(), None);
        assert_eq!(min_heap.peek(), None);
        assert_eq!(max_heap.replace(1), None);
        assert_eq!(min_heap.replace(1), None);
    }
}
