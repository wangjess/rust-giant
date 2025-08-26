
#[derive(Debug)] // so we can inspect the state in a minute
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}



fn main() {
    // Vectors can only store values of the same type

    let mut v = Vec::new();

    let _v1 = vec![1, 2, 3];
    let _v2: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Referencing values via index or by using the 'get' method.

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Getting an impossible value -> How does Rust behave?

    //let _does_not_exist: &i32 = &v[100]; // Program will crash because this doesn't exist
    let does_not_exist: Option<&i32> = v.get(100); // Will return None without panicking

    println!("DNE is {does_not_exist:?}");   

    // Iterating over values in a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // Iterating over mutable references
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // Using enums of different types to store in a vector
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        dbg!(i);
    }
}