fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    
    let condition = true;
    let value = if condition { "is an expression" } else { "is not an expression" };

    println!("If {value}");

    loops();
    labelled_loops();
    while_loops();
    while_collection();
    for_collection();
    for_range();
}

fn loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn labelled_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loops() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("FIN")
}

fn while_collection() {
    let a = [1,2,3,4,5];
    let mut index = 0;

    while index < a.len() {
        println!("Current value is: {}", a[index]);
        index += 1;
    }
}

fn for_collection() {
    let a = [12,3,4,5,6];

    for element in a {
        println!("Current value is: {element}")
    }
}

fn for_range() {
    for number in (1..4).rev() {
        println!("[Range] {number}!");
    }
}