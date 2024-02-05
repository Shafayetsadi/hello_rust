/*
Ownership is a set of rules that govern how a Rust program manages memory.
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.
*/

fn main() {
    println!("Hello, world!");

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let x: i32 = 5;
    let _y: i32 = x; // x is copied to y
    println!("{}", x); // x is still valid

    let s1 = String::from("hello");
    let _s2 = s1; // s1 is moved to s2, s1 -> not valid -> only one owner at a time
    // println!("{}, world!", s1); // error: value borrowed here after move

    let _s3 = _s2.clone(); // s2 is cloned to s3, s2 is still valid
    println!("{}, world!", _s2); // s2 is still valid

    // Ownership and Functions
    let str = String::from("hello");
    takes_ownership(str);
    // println!("{}", str); // error: value borrowed here after move

    let num: i32 = 5;
    make_copy(num);
    println!("{}", num); // output: 5

    // Return Values and Scope
    let str = gives_ownership();
    println!("{}", str);

    let s = takes_and_gives_back(str);
    println!("{}", s);

    // References and Borrowing
    /*
    The ampersand `&` gives us a reference without taking ownership of the value.
    Borrowing means that certain scope can use the value but does not own it.
    */
    let str = String::from("Shafayet");
    let len = length(&str); // str is borrowed and immutable by default
    println!("The length of '{}' is {}", str, len);

    // Dangling References
    /*
    Rust ensures that references will never be dangling references.
    The compiler will not allow the creation of dangling references.
    */
    // let ref_to_nothing = dangle(); // error: missing lifetime specifier

    // The Slice Type
    let mut str = String::from("hello, world!");
    let word = first_word(&str);
    println!("The first word is: {}", &str[..word]);
    str.clear(); // this empties the String, making it equal to ""
    println!("{}", word); // output: 6

    // String Slices
    str = String::from("hello, world!");
    let hello = &str[0..5];
    let world = &str[7..12];
    println!("{} {}", hello, world);

}

fn takes_ownership(str: String) {
    println!("{}", str);
} // some_string goes out of scope and `drop` is called. The backing memory is freed.

fn make_copy(x: i32) {
    println!("{}", x);
} // x goes out of scope, nothing happens as i32 is a Copy type

fn gives_ownership() -> String {
    let str = String::from("hello");
    str
} // str goes out of scope and is returned to the calling function

fn takes_and_gives_back(str: String) -> String {
    str + " world!"
} // str goes out of scope and is returned to the calling function

fn length(str: &String) -> usize {
    str.len()
} // str goes out of scope, nothing happens as it was borrowed

// fn dangle() -> &String { // error: missing lifetime specifier
//     let str = String::from("hello");
//     &str
// } // str goes out of scope and the reference will be dangling

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // convert string to array of bytes
    for (i, &byte) in bytes.iter().enumerate() { // iterate over the array of bytes
        if byte == b' ' { // if the byte is a space
            return i; // return the index
        }
    }
    s.len()
} // s goes out of scope, nothing happens as it was borrowed