// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("Hello "); 
    println!("Length of string is {}", hello.len()); 

    hello.push('W'); //pushes char to end of string
    hello.push_str("orld"); //Appends str to end of string. Function only takes str and not String type
    println!("{}", hello);

    //returns bool if substring is found
    println!("Contains 'World' {}", hello.contains("World")); 
    
    //finds and replaces substring, doesn't do anything if substring isn't found in string.
    println!("Replace: {}", hello.replace("Hello", "There")); 

    // Loop through string by whitespace
    for word in hello.split_whitespace() { //this function splits the string into substrings
        println!("{}", word);
    }

     // Create string with capacity
    let mut s = String::with_capacity(10); //will resize if more char or strings are pushed onto String
    s.push('a');
    s.push('b');

    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    s.push_str("cdefghijklmnopqrstuvwxyz");

    // Assertion testing
    assert_eq!(26, s.len()); //will throw error and terminate program in terminal if not equal otherwise will do nothing
    assert_eq!(26, s.capacity());

    println!("{}", s);
}
//capacity() prints string total bytes
//is_empty() returns true if string has length 0, false otherwise