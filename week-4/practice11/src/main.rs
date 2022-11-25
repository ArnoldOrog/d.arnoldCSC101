

fn main() {
    let a = 2;  // bit presentation is 10
    let b = 3; // bit presentaion is 11

let mut result:i32;

result = a & b ;
println!("(a % b --> {} ", result);

result = a | b ;
println!("(a | b)--> {}" , result);

result = a ^ b;
println!("(a ^ b)--> {}", result);

result = !b;
println!("(!b)--> {}",result);

result = a << b;
println!("(a << b) --> {}",result);

result = a >> b;
println!("(a >> b) --> {}",result);


}

