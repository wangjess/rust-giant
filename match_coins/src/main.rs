#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
    Florida,
    Michigan,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enums can also have methods
impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("message method");
    }
}

// Option enum: the value could be something or it could be nothing
// Rust doesn't have a NULL feature in its language!
// 2009 Tony Hoare, inventor of null
// the Option<T> enum is included by default.

fn main() {
    // enum
    let my_penny = Coin::Penny;

    let cents = value_in_cents(my_penny);

    println!("{cents}");

    // call enum method
    let q = Message::Quit;
    q.call();

    // Option<T>
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    // this will not compile because it's
    // trying to add an i8 to an Option<i8>
    /*
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
    */

    // note: must convert Option<T> to a T value before you can perform T operations.

    value_in_cents(Coin::Quarter(UsState::Alaska));
    value_in_cents(Coin::Quarter(UsState::Florida));
    value_in_cents(Coin::Quarter(UsState::Michigan));

    let five = Some(5);
    dbg!(five);
    let six = plus_one(five);
    dbg!(six);
    let none = plus_one(None);
    dbg!(none);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
