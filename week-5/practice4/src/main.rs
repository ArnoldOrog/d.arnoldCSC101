use std::io;
fn main() {
  let mut input1 = String::new();
  let mut input2 = String::new();

  println!("Enter your name:");
  io::stdin()
  .read_line(&mut input1)
  .expect("not a valid string");

println!("enter your age:");
io::stdin()
.read_line(&mut input2)
.expect("not a valid string");
let age:i32 = input2.trim().parse().expect("not a valid number");

if age >= 18{
    println!("Welcome to the party {}",input1);
}
else{
    println!("Oops, you're too young to enter at {}",input1);
}

}
