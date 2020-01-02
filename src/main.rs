use minmaxheap::Heap;

fn main() {
    let mut heap = Heap::new("max", 10).expect("Something did not work");
    heap.add(20);
    heap.add(10);
    heap.add(5);
    heap.add(100);
    heap.add(2);
    heap.add(40);
    heap.add(40);
    println!("{}", heap);

    heap.invert();
    println!("{}", heap);
}
