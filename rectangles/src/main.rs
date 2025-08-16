#[derive(Debug)] // this allows pretty-print of struct data
struct Rectangle {
    width: u32,
    height: u32,
}

// this uses a Method
impl Rectangle {
    // uses self. &self is short for self: &Self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("--- Rectangles ---");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        //area(&rect1)
        rect1.area() // method syntax
    );

    println!("rect1 is {rect1:?}");
    dbg!(&rect1);

    if rect1.width() { 
        println!("The width of rect1 is {}", rect1.width);
    }

    // methods with more parameters :)
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // calling associated function, which in this case
    // just creates an instance of Rectangle using the param as both height + width
    let sq = Rectangle::square(3);

    dbg!(&sq);
}


// regular function
/*
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
*/