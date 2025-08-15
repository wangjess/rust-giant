fn main() {
    println!("--- Slices ---");

    let my_string = String::from("hello world");

    let word = first_word(&my_string[0..6]);

    println!("{word}");

    // arrays
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2,3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}