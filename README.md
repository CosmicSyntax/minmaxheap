# Binary Heap
 A Binary Heap is either Min Heap or Max Heap. In a Min Binary Heap, the key at root must be minimum among all keys present in Binary Heap. The same property must be recursively true for all nodes in Binary Tree. Max Binary Heap is similar to MinHeap, except the root must be maximum among all keys present.

## Example
The following piece of code uses the library for min heap
```rust
// Create a instance of the Heap
let mut test = Heap::new("min", 6).expect("Something did not work");

// Add value into the binary tree in no particular order
test.add(20);
test.add(10);
test.add(5);
test.add(100);
test.add(2);
test.add(40);

// Print out the results
println!("{}", test);
```
The result output is
```bash
Number of nodes: 6
---------------------------------------
Node: 2 Left: 5 Right: 10
---------------------------------------
Node: 5 Left: 100 Right: 20
---------------------------------------
Node: 10 Left: 40
```

The following piece of code uses the library for max heap
```rust
// Create a instance of the Heap
let mut test = Heap::new("max", 6).expect("Something did not work");

// Add value into the binary tree in no particular order
test.add(20);
test.add(10);
test.add(5);
test.add(100);
test.add(2);
test.add(40);

// Print out the results
println!("{}", test);
```
The result output is
```bash
Number of nodes: 6
---------------------------------------
Node: 100 Left: 20 Right: 40
---------------------------------------
Node: 20 Left: 10 Right: 2
---------------------------------------
Node: 40 Left: 5
```