use qndr;


#[cfg(test)]
#[test]
fn a() {

let r = qndr::alphabets_with_symbols(&String::from("?abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"),&String::from("?"));
    assert_eq!(true,r);
}
#[cfg(test)]
#[test]

fn b() {  

let r = qndr::alphabets_with_symbols(&String::from("abc!dew@s"),&String::from("!@#"));
assert_eq!(true,r);
}
#[cfg(test)]
#[test]

fn c() {
 //no symbols allowed but dews allowed and used
let r = qndr::alphabets_with_symbols(&String::from("dews!@#$%^&*()"),&String::from("!@#$%^&*()"));
assert_eq!(true,r);
}
#[cfg(test)]
#[test]

fn d() {
 //? allowed but * used
let r = qndr::alphabets_with_symbols(&String::from("*ab5432cdews"),&String::from("?"));
    assert_eq!(false,r);
}
#[cfg(test)]
#[test]

fn e() {
 //abc allowed as symbols
let r = qndr::alphabets_with_symbols(&String::from("abc*****"),&String::from("**")); //lets see if repeating in pattern cause prob
assert_eq!(true,r);
}
#[cfg(test)]
#[test]

fn f() {

let r = qndr::alphabets_with_symbols(&String::from("_abcd"),&String::from("_"));
assert_eq!(true,r);
}
#[test]
fn g() {

let r = qndr::alphabets_with_symbols(&String::from("!@#$%^&*()_+ABCDEF"),&String::from("!@#$%^&*()_+"));
assert_eq!(true,r);
}
#[test]
fn h() {

let r = qndr::alphabets_with_symbols(&String::from("`~!@#$%^&*()_+abcdews"),&String::from("`~!@#$%^&*()_+"));
    assert_eq!(true,r);
}

