use qndr;


#[cfg(test)]
#[test]
fn a() {

let r = qndr::get_non_numbers(&String::from("abcdefghijlkmnopqrstuvwxyzABCDEFGHIJKLMNOPQRESTUVWXYZ1234567890"));
    assert_eq!("abcdefghijlkmnopqrstuvwxyzABCDEFGHIJKLMNOPQRESTUVWXYZ",r);
}
#[cfg(test)]
#[test]

fn b() {

let r = qndr::get_non_numbers(&String::from("abc!de3w7s"));
 assert_eq!("abc!dews",r);
}
#[cfg(test)]
#[test]

fn c() {

let r = qndr::get_non_numbers(&String::from("!@#$%^&*()34dews"));
    assert_eq!("!@#$%^&*()dews",r);
}
#[cfg(test)]
#[test]

fn d() {

let r = qndr::get_non_numbers(&String::from(" "));
    assert_eq!(" ",r); // the " " is a non alphabet
}
#[cfg(test)]
#[test]

fn e() {

let r = qndr::get_non_numbers(&String::from(""));
assert_eq!("",r);
}
#[cfg(test)]
#[test]

fn f() {

let r = qndr::get_non_numbers(&String::from("_1234ccc"));
assert_eq!("_ccc",r);
}
#[test]
fn g() {

let r = qndr::get_non_numbers(&String::from("!q2"));
    assert_eq!("!q",r);
}
#[test]
fn h() {

let r = qndr::get_non_numbers(&String::from("5`~!@#$9%^&*()_+abc"));
    assert_eq!("`~!@#$%^&*()_+abc",r);
}

