
/*
    Primtive Types:
    Integers: u8,i8,u16,i16,u32,i32,u64,i64,u128,i128(number of bits taken in memory) u = Non negative Values
    Floats: f32,f64
    Boolean: (bool)
    Characters: (char)
    Tuples
    Arrays
*/

pub fn run(){
    //Default is i32
    let num = 1;

    //Default is f64
    let pi = 3.142;

    //Add explicit type
    let large_num: i64 = 1240432429;

    //FindMaxSize
    println!("Max i32 is {}",std::i32::MAX);
    println!("Max i32 is {}",std::i64::MAX);

    //Boolean
    let is_active:bool = true;

    //Get boolean from expression
    let is_greater = 10 > 11;

    let a1 = 'C';
    let face = '\u{1F600}';

    println!("{:?}",(num,pi,large_num,is_active,is_greater,a1,face));
}
