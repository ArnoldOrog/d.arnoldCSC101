fn main() {
let fullname = "Pan    Atlantic University";
println!();//another way of doing \n
println!("Name:     {}",fullname);
println!();
println!("before trim ");
println!("length is {}",fullname.len());
println!();
println!("after trim      ");
println!("length is {}",fullname.trim().len());




}

//DRY MEANS DONT REPEAT YOURSELF