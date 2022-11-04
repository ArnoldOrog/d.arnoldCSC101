fn main() {
   let name: &str = "aisha lawal";
   let uni: &str = "Pan Atlantic University";
let addr: &str = "Km 52 leeki-epe expressway, Ibeju lekki, Lagos";
println!("Name: {}",name);
println!("University:{}. \n address: {}",uni,addr);


let department:&'static str = "Computer Science";
let school:&'static str = "School of Science And technology";
println!("Department: {}, \n School: {}",department,school);
}

