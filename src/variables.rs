pub fn run() {
    //Rust defaults to i32 for integers or f64 for floats if you dont specify
    //all types include i8 - i128, u8 - u128, f32 - f64, char, and bool
    //All variables are immutable by default

    let name = "Bob"; //creats a str "Bob"  Since we didnt define name as mut, if we try to modify the variable, Rust will throw an error
    let mut age = 21; //creates a i32 21
    println!("My name is {} and I am {} years old", name, age);
    age = 36;
    println!("My name is {} and I am {} years old", name, age);    

    //if you want to assign your own type
    let urage: i64 = 32; //sets age to type i64 
    println!("Your age is {}", urage);

    //define constants 
    const ID: i32 = 469;
    println!("My ID number is {}", ID);

    //Assign multiple variables
    let (my_name, my_age) = ("Will Smith", 55);
    println!("A famous actor is named {}, and is {} years old", my_name, my_age);
}