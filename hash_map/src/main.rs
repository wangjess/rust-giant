
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Get item from hash map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{score}");

    // Iterating through hash map
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them
    // see what compiler error you get!

    scores.insert(String::from("Blue"), 30);

    // Adding a K&V only if a key isn't present
    let exist = scores.entry(String::from("Yellow")).or_insert(50);
    dbg!(exist);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

    // By default, HashMap uses a function called SipHash and can provide resistance to DoS attacks
    // involving hash tables. You can use different hashers if this is too slow.
    // A hasher is a type that implements the BuildHasher trait.

    // Two Types of Errors: Recoverable & Unrecoverable
    // Unrecoverable errors are always symptoms of bugs, so we stop the program
    // Recoverable error is like a File not found error. We should report the problem to the user + retry the operation.
    // Other languages don't distinguish between these two types of errors and use Exceptions as a catch all.
    // Rust uses Result<T, E> for recoverable errors and panic! macro for unrecoverable errors.
}
