/*
A struct, or structure, is a custom data type that lets us name and
package together multiple related values that make up a meaningful group. It
is similar to a tuple, but a struct has names for its fields.
*/
struct Student {
    name: String,
    age: u8,
    department: String,
    year: u8,
    email: String,
}

struct Color(i32, i32, i32);
struct Vector(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.height > rect.height && self.width > rect.width
    }
}

struct AlwaysEqual; // unit-like struct

fn main() {
    println!("Hello, world!");
    let mut shafayet = Student {
        name: String::from("Shafayet"),
        age: 20,
        department: String::from("ECE"),
        year: 2,
        email: String::from("shafayet@gmail.com"),
    };
    println!("Name: {}", shafayet.name);
    println!("Age: {}", shafayet.age);
    println!("Department: {}", shafayet.department);
    println!("Year: {}", shafayet.year);
    println!("Email: {}", shafayet.email);

    shafayet.age = 21;
    println!("Age now: {}", shafayet.age);

    let _sadi = Student {
        name: String::from("Sadi"),
        email: String::from("sadi@gmail.com"),
        ..shafayet
    };

    let _black = Color(0, 0, 0);
    let _origin = Vector(0, 0, 0);

    let _subject = AlwaysEqual;

    let rect = Rectangle {
        width: 30,
        height: 40,
    };
    println!("The rectangle is : {:?}", rect); // {:?} is used to print debug information, {:#?} is used to print pretty debug information
                                               // we can also use dbg!() macro to print debug information
    println!("The area is: {}", area(&rect));
    println!("The area is: {}", rect.area());
    println!(
        "Can hold: {}",
        rect.can_hold(&Rectangle {
            width: 20,
            height: 30
        })
    );
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
