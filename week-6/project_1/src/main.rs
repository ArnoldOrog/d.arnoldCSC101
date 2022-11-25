use std::io;

fn trapezium(h:i32,b1:i32,b2:i32){
    let area = h/2* (b1 + b2) ;
    println!("area of trapezium is {}",area);
}

fn rhombus(d1:i32,d2:i32){
    let area = (1/2) * (d1 * d2);
       println!("area of rhombus is {}",area);
}

fn parrallel(b:i32,a:i32){
    let area = b * a;
    println!("area of parrallelogram is {}",area);
}

fn cube(l:i32){
    let area = 6*l^2;
 println!("area of cube is {}",area)
}

fn cylinder(r:i32,h:i32){
let area = (22/7)*(r*r)*h;

println!(" area of cylinder {:?}",area);
}

fn main(){
   //code for user to input values
    println!("Enter your first value:");
   let mut input1 = String::new();
   io::stdin()
   .read_line(&mut input1)
   .expect("invalid");
   let a:i32 = input1.trim().parse().expect("invalid");
   println!("enter second value");
   
   let mut input2 = String::new();
    io::stdin()
    .read_line(&mut input2)
    .expect("invalid");
    let b:i32= input2.trim().parse().expect("invalid");

    //code for user to pick values
println!("choose shape to find area");
println!("the shapes available are: cube,rhombus,cylinder,parallelogram and trapezium");
println!("to select shape, type the first letter of the shape: ");
println!("for cylinder please type y ");
let mut shapes = String::new();
io::stdin()
.read_line(&mut shapes)
.expect("invalid");
let shape:char = shapes.trim().parse().expect("invalid");
let cubes = 'c';
let rhombuss = 'r';
let cylinders = 'y';
let parallelograms = 'p';
let trapeziums = 't';


if shape == cubes{
    cube(a)
}
else if shape == rhombuss{
    rhombus(a,b)
}
else if shape == cylinders{
    cylinder(a,b)
}
else if shape == parallelograms{
    parrallel(a,b)
}
else if  shape == trapeziums{
  println!("  input third value");
     let mut input3 = String::new();
    io::stdin()
    .read_line(&mut input3)
    .expect("invalid");
    let c:i32 = input3.trim().parse().expect("invalid");
    trapezium(a,b,c)
};
 
}