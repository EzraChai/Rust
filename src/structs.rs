//Structs - Used to create custom data types

//Traditional Struct (like class in java)
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

//Tuple Struct
struct Colour(u8, u8, u8);

//
struct Person {
    first_name: String,
    last_name: String,
}

//Implements
impl Person {
    //Construct a person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    //Get Full Name
    fn get_full_name(&self) -> String {
        format!("{} {}",self.first_name,self.last_name)
    }

    //Set Last Name
    fn set_last_name(&mut self,last:&str) {
        self.last_name = last.to_string()
    }

    //Name to Tuple
    fn to_tuple(self) ->(String,String){
        (self.first_name,self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;

    println!("Traditional Color : {} {} {}", c.red, c.green, c.blue);

    let mut colour = Colour(255, 0, 0);

    colour.0 = 243;

    println!("Tuple Colour : {} {} {}", colour.0, colour.1, colour.2);


    let mut p1 = Person::new("Chloe", "Gan");
    p1.set_last_name("Gans");
    println!("Person 1 name: {} ", p1.get_full_name());
    println!("Person to Tuple : {:?}",p1.to_tuple());
}
