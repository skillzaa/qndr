use qndr;

#[cfg(test)]
#[test]
fn a() {
let r = qndr::allow_alphabets_only(&String::from("abcdews"));
    assert_eq!(true,r);
}
#[cfg(test)]
#[test]

fn b() {

let r = qndr::allow_alphabets_only(&String::from("abc!dews"));
// println!("{:?}",r);    
 assert_eq!(false,r);
}
#[cfg(test)]
#[test]

fn c() {

let r = qndr::allow_alphabets_only(&String::from("34dews"));
    assert_eq!(false,r);
}
#[cfg(test)]
#[test]

fn d() {

let r = qndr::allow_alphabets_only(&String::from("*abcdews"));
    assert_eq!(false,r);
}
#[cfg(test)]
#[test]

fn e() {

let r = qndr::allow_alphabets_only(&String::from(" "));
assert_eq!(false,r);
}
#[cfg(test)]
#[test]

fn f() {

let r = qndr::allow_alphabets_only(&String::from("0123456789"));
assert_eq!(false,r);
}
#[test]
fn g() {

let r = qndr::allow_alphabets_only(&String::from("abcdefghijlkmnopqrstuvwxyzABCDEFGHIJKLMNOPQRESTUVWXYZ"));
    assert_eq!(true,r);
}
#[test]
fn h() {

let r = qndr::allow_alphabets_only(&String::from("`~!@#$%^&*()_+abcdews"));
    assert_eq!(false,r);
}

