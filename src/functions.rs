pub fn run() {
    greeting("Hello", "Jane");
  
    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);
  
    // Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3, 3));
}
  
//functions that dont return anything acutually return unit type `()` which is similar to void in c++
fn greeting(greet: &str, name: &str) -> () { 
    println!("{} {}, nice to meet you!", greet, name);
}
  
fn add(n1: i32, n2: i32) -> i32 { //the arrow points to the type name you want the function to return
    return n1 + n2 //return statement doesnt have semi-colon. 
    //return keyword not needed for the function to return properly
}