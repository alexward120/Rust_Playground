//Vectors are part of the standard library and are resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    numbers[2] = 20; //reassign certain value in vector

    //Push to the end of vector
    numbers.push(5);
    numbers.push(6);

    numbers.pop(); //removes last value of vector

    println!("Vector numbers = {:?}", numbers);

    // Get single val
    println!("Single Value: {}", numbers[0]);

    // Get vector length
    println!("Vector Length: {}", numbers.len());

    // Vectors are heap allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Loop through vector values
    for x in numbers.iter() {
    println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);

}