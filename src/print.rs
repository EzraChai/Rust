pub fn run(){
    //Print to console
    println!("I love Chloe.");
    println!("Number : {}",1);

    //Basic Formatting
    println!("{} is my {}.","Chloe Gan","girlfriend");

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}","Ezra Chai","Malaysia","code");

    //Named Arguments
    println!("{name} likes to play {activity}.",name = "Ezra Chai",activity = "with Chloe Gan");

    //Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}",10,10,10);

    //Placeholder for debug traits
    println!("{:?}",(12,true,"hello"));

    //Basic Math
    println!("10 + 10 = {}",10+10);
}
