fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 100; // Modify the first element
    }
    println!( "{:?}", v);
}