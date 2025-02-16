fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let index = 10;

    match vec.get(index) {
        Some(value) => println!("Value at index {}: {}", index, value),
        None => println!("Index {} is out of bounds", index),
    }

    // safer alternative using bounds checking
    if index < vec.len() {
        println!("Value at index {}: {}", index, vec[index]);
    } else {
        println!("Index {} is out of bounds", index);
    }
} 