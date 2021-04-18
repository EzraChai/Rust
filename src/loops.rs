//Loops - used to iterate until a condition is met

pub fn run(){
    let mut count = 0;

    //Infinite loop
    loop{
        count += 1;
        println!("count = {}",count);

        if count == 20{
            break;
        }
    }


    print!("\n\n\n");
    count = 1;

    //While loop (FizzBuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("Fizz Buzz");
        }else if count % 3 == 0 {
            println!("Fizz");
        }else if count % 5 == 0 {
            println!("Buzz");
        }else{
            println!("Number : {}",count);
        }

        //Increment
        count += 1;
    }

    println!("\n\n\n");

    //For Range
    for num in 1..101 {
        if num % 15 == 0 {
            println!("Fizz Buzz");
        }else if num % 3 == 0 {
            println!("Fizz");
        }else if num % 5 == 0 {
            println!("Buzz");
        }else{
            println!("Number : {}",num);
        }
    }
}
