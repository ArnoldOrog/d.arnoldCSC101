use std::io;

fn checker(){

    let mut input = String::new();
    println!("Enter a character:");
    io::stdin()
    .read_line(&mut input)
    .expect("Invalid input");
    let ch:char = input.trim().parse().expect("failed to read input");

    if ch >= '0' && ch <= '9'{
        println!("CHaracter '{}' is a digit ",ch);
    }
    else{
        println!("character '{}' is not a digit",ch);
    }
}

fn main() {
   //calling a function
   println!("Welcome! This program checks whether a character variable contains a digit or not");
   checker()
}
