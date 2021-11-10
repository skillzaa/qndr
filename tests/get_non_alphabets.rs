use qndr;


#[cfg(test)]
#[test]
fn a() {

let r = qndr::get_non_alphabets(&String::from("abcdefghijlkmnopqrstuvwxyzABCDEFGHIJKLMNOPQRESTUVWXYZ1234567890"));
    assert_eq!("1234567890",r);
}
#[cfg(test)]
#[test]

fn b() {

let r = qndr::get_non_alphabets(&String::from("abc!de3w7s"));
 assert_eq!("!37",r);
}
#[cfg(test)]
#[test]

fn c() {

let r = qndr::get_non_alphabets(&String::from("!@#$%^&*()34dews"));
    assert_eq!("!@#$%^&*()34",r);
}
#[cfg(test)]
#[test]

fn d() {

let r = qndr::get_non_alphabets(&String::from(" "));
    assert_eq!(" ",r); // the " " is a non alphabet
}
#[cfg(test)]
#[test]

fn e() {

let r = qndr::get_non_alphabets(&String::from(""));
assert_eq!("",r);
}
#[cfg(test)]
#[test]

fn f() {

let r = qndr::get_non_alphabets(&String::from("_1234ccc"));
assert_eq!("_1234",r);
}
#[test]
fn g() {

let r = qndr::get_non_alphabets(&String::from("!q2"));
    assert_eq!("!2",r);
}
#[test]
fn h() {

let r = qndr::get_non_alphabets(&String::from("5`~!@#$9%^&*()_+abc"));
    assert_eq!("5`~!@#$9%^&*()_+",r);
}

