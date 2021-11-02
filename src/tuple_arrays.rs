//Tuples can have a max of 12 elements
use std::mem;


pub fn run_tuple() {
    let person: (&str, &str, i8) = ("John", "Doe", 67);
    println!("{} {} is {} years old", person.0, person.1, person.2);
    println!("{:?}", person); //prints whole tuple
}


pub fn run_array() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("xs = {:?}", xs); //prints out whole array

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];
    println!("ys = {:?}", ys);

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array
    println!("number of elements in array: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // Out of bound indexing causes compile error
  //  println!("{}", xs[5]);
}

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
} 