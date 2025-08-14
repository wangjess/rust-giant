fn main() {
    // mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //let spaces = "    ";
    //spaces = spaces.len();

    let _x = 2.0; // f64
    let _y: f32 = 3.0; //f32

    // addition
    let sum = 5 + 10;
    println!("{}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("{}", difference);

    // multiplication
    let product = 4 * 30;
    println!("{}", product);   

    // division
    let quotient = 56.7 / 32.2; // because float, will include decimals
    println!("{}", quotient);   
    let truncated = -5 / 3; // results in -1 because integers
    println!("{}", truncated);   

    // remainder
    let remainder = 43 % 5;
    println!("{}", remainder);

    // boolean
    let t = true;
    let _f: bool = false; // with explicit type annotation

    if t {
        println!("is true!");
    }

    // character type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{}", c);
    println!("{}", z);
    println!("{}", heart_eyed_cat);

    // compound types: tuples & arrays
    // tuples can have different types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // arrays must all be the same type
    let a = [1, 2, 3, 4, 5];
    let b: [u32; 5] = [1, 2, 3, 4, 5];
}
