fn main() {
    println!("Hello, world! 2");

    let numbers_vector: Vec<_> = vec![1, 2, 3].iter().map(|item| item + 10).collect();

    println!("{:?}", numbers_vector);
}
