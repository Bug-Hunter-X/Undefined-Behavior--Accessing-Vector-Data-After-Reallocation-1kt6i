fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let ptr = vec.as_ptr();

    // This is undefined behavior!  The vector's internal buffer might be reallocated.
    // Accessing `ptr` after this is dangerous and may cause a crash.
    vec.push(4); 

    println!("The first element is: {}", unsafe { *ptr });
}