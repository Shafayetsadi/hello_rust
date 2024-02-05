const _N: i32 = 10000010; // constant declaration
fn main() {
    println!("Hello, world!");

    // Variables and Mutability
    let x: i32 = 5; // immutable
    let mut y: i32 = 5; // mutable
    y += 5;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    // Shadowing
    {
        let x: i32 = 5; // outer x is shadowed by the inner x
        let x = x + 1;
        println!("The value of x is: {}", x);
    }

    // Data Types
    let mut _num: i8 = 42;
    _num += 1;
    let _num: u8 = 42;
    /*
    Integer Types:
    * Signed: i8, i16, i32, i64, i128, isize
    * Unsigned: u8, u16, u32, u64, u128, usize
    */

    // Literals
    /*
    In rust, we can use _ to improve readability. It has no effect on the
    value of numeric literals.
    */
    let _decimal = 10_000; // 10,000
    let _hex = 0xff; // 255
    let _octal = 0o77; // 63
    let _binary = 0b1111_0000; // 240
    let _byte = b'A'; // 65

    // Floating-Point Types
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // Boolean Type
    let _t = true;
    let _f: bool = false;

    // Character Type
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // Compound Types

    // Tuple
    /*
    Tuple is a compound type that groups together a number of values with
    different types. It has a fixed length. Once declared, it cannot grow or
    shrink in size.
    */
    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple: {:?}", _tup); // (500, 6.4, 1), {:?} is a debug formatter

    let (_x, _y, _z) = _tup; // destructuring the tuple
    let _a = _tup.0; // accessing the first element of the tuple
    let _b = _tup.1; // accessing the second element of the tuple
    let _c = _tup.2; // accessing the third element of the tuple

    // Array
    /*
    Array is a compound type that groups together elements of the same type.
    It is stored in a contiguous memory. It has a fixed length.
    */
    let _arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", _arr); // [1, 2, 3, 4, 5]

    println!("The first element: {}", _arr[0]);
    println!("The last element: {}", _arr[4]);
    let _arr = [3; 5]; // [3, 3, 3, 3, 3]

    // Statements and Expressions
    /*
    In Rust, statements are instructions that perform some action and do not
    return a value. Whereas, expressions evaluate to a resulting value.Expressions do
    not include ending semicolons. If you add a semicolon to the end of an
    expression, you turn it into a statement, which will then not return a
    value.
    */
    let _x = 5; // statement
    let _y = x + 5; // expression inside a statement
    let _y = {
        // block
        let x = 3;
        x + 1 // expression
    };

    // Functions
    hello();
    hello_name("Shafayet");
    println!("The sum of 5 and 10 is: {}", add(5, 10));

    // Control Flow
    let number: i32 = 5;
    if number < 5 {
        println!("The number is less than 5");
    } else if number > 5 {
        println!("The number is greater than 5");
    } else {
        println!("The number is 5");
    }

    let condition = true;
    let number: i32 = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // Loop
    let mut counter: i32 = 0;
    let result: i32 = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // break with a value
        }
    };
    println!("The result is: {}", result);

    counter = 0;
    loop {
        counter += 1;
        if counter%2 == 0{
            continue;
        }
        if counter >= 10 {
            break;
        }
        println!("The value of counter is: {}", counter);

    }

    // loop with a label
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            break 'outer; // break the outer loop
        }
        println!("This point will never be reached");

    }
    let mut number: i32 = 0;
    while number < 5 {
        println!("The value of number is: {}", number);
        number += 1;
    }
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    for element in arr.iter() { // iter() returns an iterator
        print!("{} ", element);
    }
    println!();
    for number in (1..5).rev() {
        print!("{}! ", number); // 4! 3! 2! 1!
    }
    println!();
}

fn hello() {
    println!("Hello, world!");
}
fn hello_name(name: &str) {
    println!("Hello, {}!", name);
}
fn add(x: i32, y: i32) -> i32 {
    x + y // expression
}
