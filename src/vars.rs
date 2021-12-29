// Variables hold primitive data or references to data
// Variables are immutable by default - Can't be reassigned by default
// Rust is a block-scoped langauge - Set variable in function, only pertains to that scope

pub fn run(){
    //Can't reassign, immutable by default
    let name = "Kenny";

    //Use mut to make variables mutable
    let mut age = 24;
    println!("{} is learning Rust! He is {}.", name, age);

    age = age + 1;
    println!("Next year {} will be {}", name, age);

    // Define constant - need to assign a variable type such as i32 (integer 32 bit)
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple variables at once
    let (my_name, my_age) = ("Kenny", 24);
    println!("My name: {} and My age: {}", my_name, my_age);
}