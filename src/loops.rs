pub fn run() {
    //Infinite loop
    let mut count = 0;
    loop {  //must have a break statement loop can reach
        count += 1;
        if count == 20 {
            break;
        }
    }
    assert_eq!(20, count);

    //While loop
    count = 0;
    while count != 30 {
        count += 1;
    }
    assert_eq!(30, count);

    //For Range loop
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {  //using for n in 1..=101 includes 101 in range
        if n % 5 == 0 {
            println!("{} multiple of 5", n);
        }
    }

    //Loops using iterators can be found in vectors.rs
}

