fn main() {
    let mut v = vec![1, 2, 3];
    let mut x = v.clone();
    let ptr = x.as_mut_ptr();
    unsafe {
        *ptr = 10; 
    }
    println!("x: {:?}", x);
    println!("v: {:?}", v); // v remains unchanged
}