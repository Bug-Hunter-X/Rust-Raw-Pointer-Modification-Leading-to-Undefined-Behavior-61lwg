fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 100; // Safe modification using indexing
    println!( "{:?}", v);
}