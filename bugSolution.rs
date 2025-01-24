fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Safe way to access elements using indexing
    println!("The first element is: {}", vec[0]);

    vec.push(4);

    // Accessing elements using an iterator is also safe
    for i in &vec {
        println!("Element: {}", i);
    }
} 