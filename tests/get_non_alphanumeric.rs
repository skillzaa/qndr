use qndr;


#[cfg(test)]
#[test]
fn a() {

let r = qndr::get_non_alphanumeric(&String::from("abcdefghijlkmnopqrstuvwxyzABCDEFGHIJKLMNOPQRESTUVWXYZ1234567890"));
    assert_eq!("",r);
}
#[cfg(test)]
#[test]

fn b() {

let r = qndr::get_non_alphanumeric(&String::from("abc!de3w7s"));
 assert_eq!("!",r);
}
#[cfg(test)]
#[test]

fn c() {

let r = qndr::get_non_alphanumeric(&String::from("!@#$%^&*()34dews"));
    assert_eq!("!@#$%^&*()",r);
}
#[cfg(test)]
#[test]

fn d() {

let r = qndr::get_non_alphanumeric(&String::from(" "));
    assert_eq!(" ",r); // the " " is a non alphabet
}
#[cfg(test)]
#[test]

fn e() {

let r = qndr::get_non_alphanumeric(&String::from(""));
assert_eq!("",r);
}
#[cfg(test)]
#[test]

fn f() {

let r = qndr::get_non_alphanumeric(&String::from("_1234ccc"));
assert_eq!("_",r);
}
#[test]
fn g() {

let r = qndr::get_non_alphanumeric(&String::from("!q2"));
    assert_eq!("!",r);
}
#[test]
fn h() {

let r = qndr::get_non_alphanumeric(&String::from("5`~!@#$9%^&*()_+abc"));
    assert_eq!("`~!@#$%^&*()_+",r);
}

