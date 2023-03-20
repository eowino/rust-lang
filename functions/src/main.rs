fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}"); // 4
    
    another_function(6, 'h'); // 6h
    
    let value = double(10) + five();
    println!("The value of y is: {value}"); // 25
}

fn another_function(x: i32, unit_label: char) {
    println!("Another function. {x}{unit_label}");
}

fn double(x: i32) -> i32 {
    x * 2
}

fn five() -> i32 {
    5
}