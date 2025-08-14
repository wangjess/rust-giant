use std::io;

fn main() {
    println!("Fibonacci Program");

    loop {
        println!("Please input a number.");

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // fibonacci calculations here
        let fib_num = fibonacci(number);
        println!("The fibonacci value is: {fib_num}");
    }
}

fn fibonacci(num: i32) -> i32 {
    if num == 0 {
        0
    }
    else if num == 1 {
        1
    }
    else {
        fibonacci(num - 1) + fibonacci(num - 2)
    }
}