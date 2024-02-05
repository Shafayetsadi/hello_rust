/*
Enums in Rust are a way to define a type by enumerating its possible values.
Enums are useful whenever you have a fixed set of values that you know your program needs to have.
*/

enum Direction {
    North,
    South,
    East,
    West,
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

enum Option<T> {
    None,
    Value(T),
}

fn main() {
    println!("Hello, world!");

    let direction = Direction::North;
    match direction {
        Direction::North => println!("Go North"),
        Direction::South => println!("Go South"),
        Direction::East => println!("Go East"),
        Direction::West => println!("Go West"),
    }

    let _circle = Shape::Circle(3.14);

    let num = Option::Value(5);
    match num {
        Option::None => { println!("No value") }
        Option::Value(i) => { println!("Value: {}", i) }
    }


}
