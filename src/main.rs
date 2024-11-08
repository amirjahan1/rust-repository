#![allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
 

// A struct with two fields 

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

     
 
}