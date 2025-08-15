struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

// uses String instead of &str string slice type. Why?
// we want each struct instance to own all of its data
// and for that data to be valid for as long as the entire struct is valid
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
// note: structs can store refs to data owned by something else
// but doing that requires the use of Lifetimes, a Rust feature.
// lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.

fn main() {
    println!("--- Structs ---");

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let old_email = &user1.email;
    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(String::from("newguy@email.com"), String::from("newguy123"));

    println!("{0}", {user2.email});
    println!("{0}", {user2.username});

    // regular syntax to create user3
    // must be commented out because active and username are heap-allocated. they will be moved.
    /*
    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("someone@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    */

    // struct update syntax to create user_3
    let user_3 = User {
        email: String::from("someone@example.com"),
        ..user1
    };

    println!("--- Tuple Structs ---");
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("--- Unit-Like Structs ---");
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    // field init shorthand
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

