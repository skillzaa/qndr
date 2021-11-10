use qndr;


fn main(){
let result_a = qndr::begin_with_alphanumeric(&String::from("!!!!Hayyy"));
assert_eq!(false,result_a);

let result_b = qndr::begin_with_alphabet(&String::from("_1234?"));
assert_eq!(false,result_b);

let result_c = qndr::allow_alphabets_only(&String::from("0123456789"));
assert_eq!(false,result_c);

let result_d = qndr::allow_alphanumeric_only(&String::from(" "));
assert_eq!(false,result_d);

println!("All operations completed...");
}
