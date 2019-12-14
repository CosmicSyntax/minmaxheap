use minmaxheap::Tree;

fn main() {
    let test = Tree::new("min", 10).expect("Something did not work");

    println!("{}", test.heap);
}
