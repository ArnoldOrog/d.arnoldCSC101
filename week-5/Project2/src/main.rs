use std::io;

fn main() {
   //age
   let mut boolss:bool;
    println!("Good Morning!");
    println!(" Please enter your age:");
    let mut x = String::new();
    io::stdin()
    .read_line(&mut x)
    .expect("hg");
    let age:f64 = x.trim().parse().expect("h");
    // println!("{}",age);
    
//Experience
println!("\nDo you have experience?");
println!(" If you do type y for yes");
println!("If not type n for no");
let mut input = String::new();
io::stdin()
.read_line(&mut input)
.expect("invalid");
let expr:char = input.trim().parse().expect("j");
let exp =  'y';
let mut bools = false;
if exp == expr {
    bools = true;
}

//the code
if !bools  {
    println!("The incentive for you is N100,000");
}
else if  age > 30.0 &&  age < 40.0 && bools{
    println!("The icentive for you is N1,480,000");
}
else if age < 28.0 && bools {
    println!("The incentive for you is N1,300,000");
}
else if age >= 40.0 && bools {
    println!("The incentive for you is N1,560,000")
}
} 




