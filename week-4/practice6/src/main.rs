fn main() {
let n1 = "eletrical".to_string(); //to_string takes the data in the in var and turns it to a string
let n2 = " eletronic".to_string();
let n3 = " engineering".to_string();
let n4 = n1 + &n2 + &n3; //you have to put the '&' anytime you you have multiple variables in a variable output
// acbout eletrical enginnering
println!("\nThe {} is informed by the aspiration to train electrical/eletronic engineering proffesionals in the areas of design, building and maintenance of eletrical control systems",n4);// you have to add "{}" to call the variable or it will show the error 'Argument not used'!!!!!

let w1 = "Computer".to_string();
let w2 = " Science".to_string();  
let w3 = w1 + &w2;
println!();// this empty println is used to break the line but its easier to use the \n method
println!("{} is aimed at devloping competent, creative, innovative, enterprenuerial and ethically-minded persons , capable of creating value in the diverse fields of computer science. ",w3);









}
