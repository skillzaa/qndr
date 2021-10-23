mod qndr;
use qndr::Qndr;

impl Qndr for f64 { }

fn main() {
    let m:f64 = 4.0;

let r = m.alphabets_only(String::from("alphabetsonly"));
println!("Result is {}",r);

}
