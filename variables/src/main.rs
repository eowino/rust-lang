fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = 3 + x;
    println!("The value of x is: {x}");
    
    {
        let x = true;
        println!("The value of x in the inner scope: {x}");
    }

    println!("The final value of x: {x}");
}