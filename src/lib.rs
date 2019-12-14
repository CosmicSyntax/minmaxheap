use std::fmt;

pub struct Heap {
    pub value: i32,
    pub left: Option<Box<Heap>>,
    pub right: Option<Box<Heap>>,
}

pub struct Tree<'a> {
    pub heap: Heap,
    pub kind: &'a str,
}

impl<'a> Tree<'a> {
    pub fn new(kind: &str, value: i32) -> Result<Tree, &str> {
        if kind == "max" || kind == "min" {
            Ok(Tree {
                heap: Heap {
                    value,
                    left: None,
                    right: None,
                },
                kind,
            })
        } else {
            Err("The type of heap you entered is not one of the options.")
        }
    }

    #[inline]
    pub fn add(&mut self, value: i32) {
        unimplemented!();
    }

}

fn seek<'a>(start: &'a mut Tree) -> &'a Option<Box<Heap>> {
    // An associative function to search for the next empty node
    if let None = start.heap.left {
        // Go left
        &start.heap.left
    } else if let None = start.heap.right {
        &start.heap.right
    } else {
        // If both are full... recursively go lower in the tree using fn seek
        unimplemented!();
    }
}

impl fmt::Display for Heap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let left = match &self.left {
            Some(_) => String::from("There is a value on the left branch"),
            None => String::from("None"),
        };

        let right = match &self.right {
            Some(_) => String::from("There is a value on the right branch"),
            None => String::from("None"),
        };

        write!(
            f,
            "Current Node: {}\nLeft Child: {}\nRight Child: {}",
            self.value, left, right
        )
    }
}
