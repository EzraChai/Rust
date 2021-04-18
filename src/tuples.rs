//Tuples group together values of different Types
//Max = 12 elements

pub fn run(){
    //Tuples
    let person:(&str,&str,u8) = ("Chloe Gan","Dataran Segar",16);

    println!("{} is from {} and she is {} years old.",person.0,person.1,person.2);
}
