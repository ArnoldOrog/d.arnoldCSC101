use std::io;

fn main() {
    println!("Input a: ");
    let mut first = String::new();
    io::stdin().read_line(&mut first).expect("invalid");
    let  x  : f64 = first.trim().parse().expect("not valid");

    println!(" Input b: ");
    let mut second = String::new();
    io::stdin()
    .read_line(&mut second)
    .expect("invalid");
    let y: f64 = second.trim().parse().expect("not valid");

    println!("Enter third number: ");
     let mut third = String::new();
    io::stdin()
    .read_line(&mut third)
    .expect("invalid");
    let z: f64 = third.trim().parse().expect("not valid");

    let discriminant:f64 = y*y - 4.0*x*z;
    
    if discriminant == 0.0{
        println!("There are no real roots : {:?}",discriminant);
    }
    else if discriminant >= 0.0{
        println!("It is positive {:?}", discriminant)
    }
    else if discriminant <= 0.0{
        println!("It is negative \n{:?}", discriminant)
    };
    
    println!("Discriminant \n: {:?}",discriminant);
}