use std::io;

fn main() {
    println!("\nStudent Information Management System!");

//input name
println!("\nPlease Enter your name. ");
let mut name = String::new();
io::stdin()
.read_line(&mut name)//this is like an alert in js in the sense that it allows the user input the value
.expect("Failed to read input");//this will be the output if the user does not return a string
println!("Your name is {}", name);

//input age
println!("Enter your age. ");
let mut age = String::new();// this declares an empty variable
io::stdin()
.read_line(&mut age)// this read the parameters in the variable
.expect("Failed to reaad input");
println!("Your name is: {}", name);

//input age
println!("\n Enter your age. ");
let mut age = String::new();
io::stdin().read_line(&mut age).expect("Failed to read input");
let age:i32 = age.trim()//removes all the empty spaces
.parse()//converts a string to the i,u,f speciefied e.g converts string to f32
.expect("Input not an integer");// output only if input isn't an integer
println!("Your age is : {}", age);












}
