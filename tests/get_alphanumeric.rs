use qndr;


#[cfg(test)]
#[test]
fn a() {

let r = qndr::get_alphanumeric(&String::from("abcdefghijlkmnopqrstuvwxyzABCDEFGHIJKLMNOPQRESTUVWXYZ1234567890"));
    assert_eq!("abcdefghijlkmnopqrstuvwxyzABCDEFGHIJKLMNOPQRESTUVWXYZ1234567890",r);
}
#[cfg(test)]
#[test]

fn b() {

let r = qndr::get_alphanumeric(&String::from("abc!dews"));
 assert_eq!("abcdews",r);
}
#[cfg(test)]
#[test]

fn c() {

let r = qndr::get_alphanumeric(&String::from("!@#$%^&*()34dews"));
    assert_eq!("34dews",r);
}
#[cfg(test)]
#[test]

fn d() {

let r = qndr::get_alphanumeric(&String::from(" "));
    assert_eq!("",r);
}
#[cfg(test)]
#[test]

fn e() {

let r = qndr::get_alphanumeric(&String::from(""));
assert_eq!("",r);
}
#[cfg(test)]
#[test]

fn f() {

let r = qndr::get_alphanumeric(&String::from("_1234ccc"));
assert_eq!("1234ccc",r);
}
#[test]
fn g() {

let r = qndr::get_alphanumeric(&String::from("!q2"));
    assert_eq!("q2",r);
}
#[test]
fn h() {

let r = qndr::get_alphanumeric(&String::from("5`~!@#$9%^&*()_+abc"));
    assert_eq!("59abc",r);
}

