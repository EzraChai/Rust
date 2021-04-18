//Vectors - Resizable arrays

use std::mem;


pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    //Re-assign values
    numbers[4] = 9;

    println!("{:?}",numbers);


    //Add on to vector
    numbers.push(5);
    numbers.push(6);

    println!("After adding numbers = {:?}",numbers);

    //Pop off last value
    numbers.pop();

    println!("After pop off the number = {:?}",numbers);


    //Get single value
    println!("First value = {}",numbers[0]);

    //Get vector length
    println!("Vector length = {}",numbers.len());

    //Vectors are stack allocated
    println!("Vector occupies {} bytes.",mem::size_of_val(&numbers));

    //Get Slice
    let slice:&[i32] = &numbers[1..3];
    println!("Slice: {:?}",slice);


    //Loop through values
    for num in numbers.iter(){
        println!("Num = {}",num);
    }

    //Loop and mutate values
    for number in numbers.iter_mut() {
        *number += 2;
    }
    println!("Numbers Vec : {:?}",numbers);
}

