pub fn run() {
    greeting("Hello", "Chloe Gan");

    //Bind Function values to variables
    let get_sum: i32 = add(1, 2);
    println!("Sum: {}", get_sum);

    //Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C sums : {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{}, {} nice to meet you.", greet, name);
}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
