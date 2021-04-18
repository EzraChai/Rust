pub fn run(){
    let name = "Chloe Gan";

    //mut == mutable;
    let mut age:i32 = 16;

    age = 17;

    println!("My girlfriend's name is {} and she is {} years old.",name,age);

    //Define const
    const ID:i32 = 001;
    println!("ID: {}",ID);

    //Asign multiple vars
    let (my_name, my_age) = ("Ezra Chai", 16);
    println!("Name: {}, age: {}", my_name, my_age);
}
