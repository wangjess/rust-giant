fn main() {
    println!("Hello, world!");

    let s = String::from("hello"); // s comes into scope, allocated onto heap for us

    takes_ownership(s); // s's value moves into the function... and is no longer valid here

    // this line will cause a compiler error because we moved value s already!!!!!
    //println!("{s}");

    // x comes into scope.
    // simple scalar type means it is on the STACK
    let x = 5; 

    // because i32 implements the copy trait
    // x does not move into the function, okay to use after
    makes_copy(x); 
    
    let y = x;

    println!("{x}");
    println!("{y}");

    let dummy = String::from("hola");
    println!("{dummy}");

    // dummy no longer points to a valid string. it is moved.
    let dummy_new = dummy;
    println!("{dummy_new}");

    // return multiple values
    let s1 = String::from("nihao");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}");
} // x goes out of scope, then s. because s's value was moved, nothing special occurs

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // some_string goes out of scope and 'drop' is called. the backing memory is freed.

fn makes_copy(some_int: i32) {
    println!("{some_int}");
} // here, some_int goes out of scope. nothing special happens.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}