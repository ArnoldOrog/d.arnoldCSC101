use std::io;

fn add(a:i32, b:i32) {
    let sum = a + b;

   println!("The sum of a and b is {}",sum);
}

fn main(){
let mut input1 = String::new();
io::stdin()
.read_line(&mut input1)
.expect("Not a valid input");
let a:i32 = input1.trim().parse().expect("failed to read input");

let mut input2 = String::new();
io::stdin()
.read_line(&mut input2)
.expect("not a valid input");
let b:i32 = input2.trim().parse().expect("failed to read input");

//call add to add functions
add(a, b);
}
