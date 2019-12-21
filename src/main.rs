use minmaxheap::Heap;

fn main() {
    let mut test = Heap::new("max", 10).expect("Something did not work");
    test.add(20);
    test.add(10);
    test.add(5);
    test.add(100);
    test.add(2);
    test.add(40);
    println!("{}", test);
}
