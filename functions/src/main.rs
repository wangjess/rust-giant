fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    // macro
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // the equivalent of this line would be "let x = 5"
    let x = five();
    println!("The value of x is: {x}");

    let a = plus_one(1);
    println!("The value of a is: {a}");

    // control flow
    let condition = true;
    let number = if condition {5} else {6};

    println!("The value of number is: {number}");

    // three types of repetition:
    // loop, while, for

    // basic loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // break
            break counter * 2; 
        }
    };

    // loop labels to disambiguate between multiple loops
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

    println!("Result is: {result}");

    // while 
    let mut number = 3;

    while number != 0 {
        println!("number: {number}");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for loop
    // a fan favorite amongst Rustaceans due to its safety and concise nature
    let array = [1, 2, 3, 4, 5];

    for element in array {
        println!("element = {element}");
    }

    // loop with rev (reverses the range)
    // prints 3, 2, 1
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!");
    

    return
}

// parameters
fn another_function(value: i32, unit_label: char) {
    println!("value is: {value}");
    println!("unit_label is: {unit_label}");
}

// there is no semicolon because it is an expression whose value we want to return
fn five() -> i32 {
    5
}

fn plus_one(val: i32) -> i32 {
    val + 1
}