fn main() {
    deep_copy_example();
}

fn invalidate_example() {
    let s1 = String::from("hello");
    let s2 = s1;
    
    // The following won't work because Rust considers s1 it as invalidated
    // We would say "s1 was 'moved' into s2"
    println!("s1 {}", s1); 
}

fn deep_copy_example() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // This copies the heap data along with the stack data

    println!("s1 {}, s2 {}", s1, s2); 
}

fn valid_example() {
    // Integers have a known size at compile time and are stored on the stack
    // so copies of the actual value are quick to make. No diff between calling/not calling clone.
    let x = 5;
    let y = x; 

    println!("x = {}, y = {}", x, y);
}