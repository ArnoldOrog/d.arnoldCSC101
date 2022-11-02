// Project I
fn main () {
    let p:f64 = 520000.00; //principal(p)
    let r:f64 = 10.0; //rate(r)
    let t:f64 = 5.0; //time(t)
    let a = p*(1.0+(r/100.0))*t; // to calculate amount
    let c = a-p; // to calculate compound intrest
    println!("amount is {}",a);
    println!("Compound intrest is {}",c);
}



//Project II
/*fn main () {
   const TOSHIBA:f64 = 450000.00;
   const  MAC:f64 = 1500000.00; 
   const HP:f64 = 750000.00;
   const DELL:f64 = 2850000.00;
   const ACER:f64 = 250000.00;
   let t:f64 = TOSHIBA * 2.0; // total amount of toshiba
   let m:f64 = MAC * 1.0; // total amount of MAC
   let h:f64 = HP * 3.0; // total amount of HP
   let d:f64 = DELL * 3.0; // total amount of Dell
   let a:f64 = ACER * 1.0; // total amount of Acer
   let sum:f64 = t + m + h + d + a; //sum
   println!("The total sales record is {}", sum);
   let average:f64 = sum/10.0; // average
   println!("The average is {}",average);
}*/

//Project III;
/*fn main() {

let p:f64 = 210000.00; // principal
let r:f64 = 10.0; //rate
let t:f64 = 3.0; //time
let a:f64 = p*(1.0-(r/100.0))*t; // fromula needed to find amount 
let d:f64= a-p; // depreciation formula
println!("depredciation is {}" , c); //print
}*/