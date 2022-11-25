//input age
println!("Enter your age. ");
let mut age = String::new();// this declares an empty variable
io::stdin()
.read_line(&mut age)// this read the parameters in the variable
.expect("Failed to reaad input");
println!("Your name is: {}", name);