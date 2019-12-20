use std::fmt;

pub struct Heap<'a> {
    pub capacity: usize,
    pub heap_size: usize,
    pub kind: &'a str,
    heap: Vec<i32>,
}

impl<'a> Heap<'a> {
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

    pub fn add(&mut self, value: i32) {
        self.heap.push(value);
        self.sort();
    }

    fn sort(&mut self) {
        if self.kind == "min" {
        } else {
        }
        unimplemented!();
    }

    fn left(&mut self, index: usize) -> i32 {
        self.heap[2 * index + 1]
    }

    fn right(&mut self, index: usize) -> i32 {
        self.heap[2 * index + 2]
    }

    pub fn peak(&mut self) -> Result<i32, &str> {
        if self.heap.len() == 0 {
            Err("There is nothing in the heap.")
        } else {
            Ok(self.heap[0])
        }
    }
}

// impl<'a> fmt::Display for Heap<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let left = self.heap[]

//         write!(
//             f,
//             "Current Node: {}\nLeft Child: {}\nRight Child: {}",
//             self.value, left, right
//         )
//     }
// }
