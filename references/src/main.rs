fn main() {
    println!("References and Borrowing!");

    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}");

    change(&mut s1);

    // rust compiler won't throw an error
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{r1}, {r2}");

    // will throw an error
    /*
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{r1}, {r2}");
    */

    // not valid, either
    /*
    let r_1 = &s; // no problem
    let r_2 = &s; // no problem
    let r_3 = &mut s; // BIG PROBLEM

    println!("{r_1}, {r_2}, and {r_3}");
    */

    // valid, because variables r_1 and r_2 won't be used after that point
    let mut s_1 = String::from("hello");

    let r_1 = &s_1; // no problem
    let r_2 = &s_1; // no problem
    println!("{r_1} and {r_2}");
    // Variables r1 and r2 will not be used after this point.

    let r_3 = &mut s_1; // no problem
    println!("{r_3}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// this will not work. references are immutable unless specified as mut
/*
fn change(some_string: &String) {
    some_string.push_str(", world");
}
*/

// this is valid due to MUTABLE references.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}