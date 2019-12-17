use minmaxheap::Tree;

fn main() {
    let mut test = Tree::new("min", 10).expect("Something did not work");

    test.add(30);
    test.add(20);

    println!("{}", test.heap.unwrap());
}
