fn main() {
    println!("Hello, world!");

    let data = "initial contents";

    let mut s = data.to_string();
    println!("{s}");

    let s1 = "initial contents".to_string();
    println!("{s1}");

    let s2 = String::from("initial contents");
    println!("{s2}");

    // Strings can be used with any properly encoded data like so:
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Appending to a string with push_str and push
    s.push_str("+added");
    //s.push_str('c'); // won't work because this is a char
    println!("{s}");

    s.push('c'); // takes in a single char
    //s.push("c"); // won't work because this is &str
    println!("{s}");

    // Different parameter
    s.push_str(data); // works because this is a &str
    //s.push_str(s1); // won't work because arg is a String
    println!("{s}");

    // Concatenation with the + Operator or the format! Macro
    let hello_string = String::from("Hello, ");
    let world_string = String::from("world!");
    let hello_world = hello_string + &world_string; // hello_string has been moved here + won't be usable anymore
    println!("{hello_world}");

    // + operator uses the add method, whose signature looks something like this:
    // fn add(self, s: &str) -> String {
    // even though the 2nd arg is actually a String, the compiler coerces the &String arg into a &str
    // this is called a deref coercion.

    // format! macro
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let final_game = format!("{tic}-{tac}-{toe}");
    // format! works like println!, but instead of printing to console, it returns a String
    println!("{final_game}");

    // Indexing into Strings are not supported
    //let h = hello_string[0];
    // this is due to how Rust stores strings in memory

    // A String is a wrapper over a Vec<u8>
    // not a 3, it is a captical Cyrillic letter Ze
    let russian_hello = String::from("Здравствуйте");
    // length is not 12, in Rust the length is 24
    // that's the # of bytes it takes to encode that string in UTF-8
    // each Unicode scalar value in that string takes 2 bytes of storage

    // Strings can be stored as: bytes, scalar values, grapheme clusters (the closest thing to letters)
    // A final reason why Rust doesn't allow us to index into a String to get a char
    // is because indexing ops are supposed to take O(1) but we can't guarantee that in Rust

    // Slicing into Strings
    let russian_slice = &russian_hello[0..4]; // valid
    //let illegal_slice = &russian_hello[0..1]; // this is not a full slice, this is only part of the 1st char

    // Iterating over Strings
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

    // Remember: Unicode scalar values may be made up of more than one byte.
    // Using grapheme clusters in Strings is by default not supported in the standard library. As it is complex.


}
