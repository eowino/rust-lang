fn main() {
    let u: u8 = u8::MIN;
    let i: i8 = i8::MAX;

    let tup: (i32, f64, i8) = (500, 2.2, 4);
    let x = tup.1;

    println!("The value of u is: {u}"); // 0
    println!("The value of i is: {i}"); // 127
    println!("The value of x is: {x}"); // 2.2
    
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // arrays are useful for fixed size collections
    
    let arr = [0; 4]; // [0, 0, 0, 0]
    let size = arr.len();
    let third = arr[2];

    println!("The value of x is: {size}"); // 4
    println!("The value of x is: {third}"); // 0
}
