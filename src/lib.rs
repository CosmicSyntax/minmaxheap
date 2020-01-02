//! # Binary Heap
//! This crate implements both min and max binary heaps.

//! # Quick Start
//! ```rust
//! use minmaxheap::Heap;
//!
//! // Instantiate a Heap
//! let mut heap = Heap::new("max", 10).expect("Something did not work");
//!
//! // Add data
//! heap.add(20);
//! heap.add(10);
//! heap.add(5);
//! heap.add(100);
//! heap.add(2);
//! heap.add(40);
//!
//! // Print into terminal
//! println!("{}", heap);
//! 
//! // Invert the heap from max to min
//! heap.invert();
//! 
//! // Print into terminal
//! println!("{}", heap);
//! ```

use std::{fmt, mem};

/// This is a struct where the binary heap is stored, along with other metadata.
/// Click on "Heap" to explore the methods for this data structure.
pub struct Heap<'a> {
    /// Initial size of the vector to allocate when struct is instantiated.
    /// You can add more than the initial size, but should avoid this for performance
    /// reasons.
    pub capacity: usize,
    /// This is the counter for the number of nodes in the heap. Counter is set to
    /// 1 as soon as the first node is added.
    pub heap_size: usize,
    /// The type of binary heap. Hold a string literal of "min" or "max".
    pub kind: &'a str,
    /// The heap vector. This is private.
    heap: Vec<i32>,
}

impl<'a> Heap<'a> {
    /// Creates a new instance of the Heap Struct.
    /// There are two kinds of implementation: min and max.
    /// # Example
    /// ```rust
    /// let mut min = minmaxheap::Heap::new("min", 10).expect("Something did not go right.");
    /// let mut max = minmaxheap::Heap::new("max", 10).expect("Something did not go right.");
    /// ```
    /// # Panic
    /// ```rust,should_panic
    /// let mut min = minmaxheap::Heap::new("wrong", 10).expect("Something did not go right.");
    /// ```
    pub fn new(kind: &str, capacity: usize) -> Result<Heap, &str> {
        if kind == "min" || kind == "max" {
            Ok(Heap {
                capacity,
                heap_size: 0,
                kind,
                heap: Vec::with_capacity(capacity),
            })
        } else {
            Err("You type of heap you specified is not supported.")
        }
    }

    /// Add nodes into the heap. The nodes do not have to be entered in any order.
    /// The add method calls the sort (a private) method which the binary heap is
    /// ordered correctly. Duplicate nodes are allowed.
    /// # Example
    /// ```rust
    /// let mut min = minmaxheap::Heap::new("min", 10).expect("Something did not go right.");
    /// min.add(10);
    /// assert_eq!(10, min.peak().unwrap());
    /// ```
    pub fn add(&mut self, value: i32) {
        self.heap.push(value);
        self.heap_size += 1;
        self.sort(self.heap_size - 1);
    }

    fn sort(&mut self, index: usize) {
        // Base case... if on the first element, do nothing
        if index > 0 {
            if self.kind == "min" {
                let parent_index = Heap::parent(index);
                if self.heap[parent_index] > self.heap[index] {
                    let (left, right) = self.heap.split_at_mut(index);
                    mem::swap(&mut left[parent_index], &mut right[0]);
                }
                self.sort(index - 1);
            } else {
                let parent_index = Heap::parent(index);
                if self.heap[parent_index] < self.heap[index] {
                    let (left, right) = self.heap.split_at_mut(index);
                    mem::swap(&mut left[parent_index], &mut right[0]);
                }
                self.sort(index - 1);
            }
        }
    }

    fn left(index: usize) -> usize {
        2 * index + 1
    }

    fn right(index: usize) -> usize {
        2 * index + 2
    }

    fn parent(index: usize) -> usize {
        if index % 2 != 0 {
            index / 2
        } else {
            index / 2 - 1
        }
    }

    /// Return the value in the top node. If min was chosen, this would be the
    /// smallest value in the binary heap. This would return the max if max
    /// was chosen.
    /// # Example
    /// ```rust
    /// let mut min = minmaxheap::Heap::new("min", 10).expect("Something did not go right.");
    /// min.add(10);
    /// min.add(40);
    /// min.add(5);
    /// assert_eq!(5, min.peak().unwrap());
    /// ```
    pub fn peak(&mut self) -> Result<i32, &str> {
        if self.heap_size == 0 {
            Err("There is nothing in the heap.")
        } else {
            Ok(self.heap[0])
        }
    }

    /// Takes the existing heap and inverts it from max to min or min to max.
    /// Under the hood, this method basically drains the vector and addes them back into
    /// the heap using the existing add method. Future update should include an optimized
    /// version of invert that doesn't take O(nlogn).
    /// # Example
    /// ```rust
    /// let mut heap = minmaxheap::Heap::new("min", 10).expect("Something did not go right.");
    /// heap.add(10);
    /// heap.add(40);
    /// heap.add(5);
    /// heap.invert();
    /// assert_eq!(40, heap.peak().unwrap());
    /// ```
    pub fn invert(&mut self) {
        // first... change min to max, or max to min
        if self.kind == "min" {
            self.kind = "max";
        } else {
            self.kind = "min";
        }

        let tmp: Vec<i32> = self.heap.drain(..).collect();
        self.heap_size = 0;

        for i in tmp.into_iter() {
            self.add(i);
        }

    }
}

impl<'a> fmt::Display for Heap<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lowest_parent = Heap::parent(self.heap_size - 1);
        let n = self.heap_size - 1;
        let mut message = String::new();

        message.push_str(&format!("Number of nodes: {} | Type: {}\n", 
            self.heap_size, self.kind));

        for i in 0..=lowest_parent {
            let left = Heap::left(i);
            let right = Heap::right(i);
            if i != lowest_parent {
                let node = self.heap[i];
                message.push_str("---------------------------------------\n");
                message.push_str(&format!(
                    "Node: {} Left: {} Right: {}\n",
                    node, self.heap[left], self.heap[right]
                ));
            } else {
                // check for children
                if right <= n {
                    // Only right
                    let node = self.heap[i];
                    message.push_str("---------------------------------------\n");
                    message.push_str(&format!(
                        "Node: {} Left: {} Right: {}\n",
                        node, self.heap[left], self.heap[right]
                    ));
                } else {
                    let node = self.heap[i];
                    message.push_str("---------------------------------------\n");
                    message.push_str(&format!("Node: {} Left: {} \n", node, self.heap[left]));
                }
            }
        }

        write!(f, "{}", message)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn min_check() {
        let mut min = Heap::new("min", 10).expect("Something did not go right.");
        min.add(100);
        min.add(50);
        min.add(30);
        min.add(10);

        assert_eq!(10, min.peak().unwrap());
    }

    #[test]
    fn max_check() {
        let mut max = Heap::new("max", 10).expect("Something did not go right.");
        max.add(100);
        max.add(50);
        max.add(30);
        max.add(10);

        assert_eq!(100, max.peak().unwrap());
    }

    #[test]
    fn invert_check() {
        let mut heap = Heap::new("max", 10).expect("Something did not go right.");
        heap.add(100);
        heap.add(50);
        heap.add(30);
        heap.add(10);
        heap.invert();

        assert_eq!(10, heap.peak().unwrap());
        assert_eq!("min", heap.kind);
    }
}
