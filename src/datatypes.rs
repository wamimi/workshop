//SCALAR DATATYPES 
fn main() {
    let company_string = "Ledgerlady"; // string type
    let rating_float=4.5; // float type
    let is_growing_boolean=true; // boolean type
    let icon_char='♥'; //unicode character type

    println!("company name is:{}",company_string);
    println!("company rating on 5 is:{}",rating_float);
    println!("company is growing :{}",is_growing_boolean);
    println!("company icon is:{}",icon_char);
    }

    fn main() {
        let result=10;// i32 by default
        let age:u32= 20;
        let sum:i32 = 5-15;
        let interest:f32=8.35;// show automatic type casting
        println!("interest is {}",interest);
        println!("result value is {}",result);
        println!("sum is {} and age is {}",sum,age);
    }

//COMPOUND DATATYPES
//tuple types

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (3,4,5);
    let (x,y,z)= tup;
    println!("the value of y is: {y}");
}
// arrays 
fn main(){
    let a = [1, 2, 3, 4, 5];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // [3, 3, 3, 3, 3]
}


       
    