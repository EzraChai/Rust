//Primitive str = Immutable fixed-length string somewhere in memory
//String = Growable, heap-allocated data structure - Use when you need to modify or own String data

pub fn run(){
    //Primitive str
    let hello = "Hello";

    //Growable String
    let mut hello2 = String::from("Hello ");

    //Get Length
    println!("String Length: {}",hello2.len());

    //Push char
    hello2.push('W');
    println!("{}",hello2);

    //Push String
    hello2.push_str("orld");
    println!("{}",hello2);


    //Capacity in bytes
    println!("Capacity = {}",hello2.capacity());

    //Check if empty
    println!("Is Empty = {}",hello2.is_empty());

    //Contains
    println!("Contains 'World' ? {}",hello2.contains("World"));

    //Replace
    println!("Replace: {} \n",hello2.replace("World","Chloe"));

    //Loop through string by whitespace
    for word in hello2.split_whitespace() {
        println!("{}",word);
    }

    //Create String with capacity
    let mut str = String::with_capacity(10);
    str.push('a');
    str.push('b');

    println!("{}",str);
    println!("{}",str.capacity());


    //Assertion Testing

    // assert_eq!(3,str.len());
    assert_eq!(2,str.len());
    // assert_eq!(9,str.capacity());
    assert_eq!(10,str.capacity());
}
