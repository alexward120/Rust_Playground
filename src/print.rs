pub fn run() {
    print!("Hello World without line"); //prints to terminal without new line

    println!("Hello World"); //prints to terminal with new line

    //Using {} allows for insertion of variables into print statement
    println!("My name is {} and my age is {}.", "Bob", 23);

    //Numbers inside {} correspond to argument order
    println!("My name is {1} and my age is {0}.", 23, "Bob");

    //You can use placeholder traits to convert certain integers
    println!("Binary: {:b}, Octal: {:o}, Hex: {:x}", 10,10,10);

    //println!("Debugger: {:?}", variable) this debugger will be seen in other files such as tuples and arrays

}