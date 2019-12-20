use minmaxheap::Heap;

fn main() {
    let mut test = Heap::new("min", 10).expect("Something did not work");
    println!("{:?}", test.peak().unwrap());
}
