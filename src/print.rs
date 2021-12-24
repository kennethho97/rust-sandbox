pub fn  run(){
    // PRint to console
    println!("Hello from the print.rs file");

    //Basic Formatting 
    println!("Number: {}", 1);

    //Positional Arguments - like an array
    println!("{0} is {1} years old. {0} is from {2}", "Kenny", 20, "SA");

    //Named Arguments
    println!(
        "{name} likes to {activity}", 
        name = "Kenny", 
        activity = "Snowboard"
    );

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //Basic Math
    println!("Yo! 10 + 10 = {}", 10 + 10);
}