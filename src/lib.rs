use std::fmt;

pub struct Heap {
    pub value: i32,
    pub left: Option<Box<Heap>>,
    pub right: Option<Box<Heap>>,
}

pub struct Tree<'a> {
    pub heap: Option<Box<Heap>>,
    pub kind: &'a str,
}

impl<'a> Tree<'a> {
    pub fn new(kind: &str, value: i32) -> Result<Tree, &str> {
        if kind == "max" || kind == "min" {
            Ok(Tree {
                heap: Some(Box::new(Heap {
                    value,
                    left: None,
                    right: None,
                })),
                kind,
            })
        } else {
            Err("The type of heap you entered is not one of the options.")
        }
    }

    #[inline]
    pub fn add(&mut self, value: i32) {
        Tree::seek(&mut self.heap, value);
    }

    fn seek(start: &mut Option<Box<Heap>>, value: i32) {
        // An associated function to search for the next empty node

        if let None = start.as_mut().unwrap().left {
            // Check left
            start.as_mut().unwrap().left = Some(Box::new(Heap {
                value,
                left: None,
                right: None,
            }))
        } else if let None = start.as_mut().unwrap().right {
            // Check right
            start.as_mut().unwrap().right = Some(Box::new(Heap {
                value,
                left: None,
                right: None,
            }))
        } else {
            // Recursive find open if both left and right are occupied
            Tree::seek(&mut start.as_mut().unwrap().left, value);
            Tree::seek(&mut start.as_mut().unwrap().right, value);
        }
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
