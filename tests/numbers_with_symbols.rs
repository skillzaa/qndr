use qndr;


#[cfg(test)]
#[test]
fn a() {

let r = qndr::numbers_with_symbols(&String::from("?1234567890"),&String::from("?"));
    assert_eq!(true,r);
}
#[cfg(test)]
#[test]

fn b() {
    // !@# are allowed but # not used  --its false since abc etc are used but not allowed  

let r = qndr::numbers_with_symbols(&String::from("abc!dew@s"),&String::from("!@#"));
assert_eq!(false,r);
}
#[cfg(test)]
#[test]

fn c() {
 //no symbols allowed but dews allowed and used
let r = qndr::numbers_with_symbols(&String::from("34dews"),&String::from("dews"));
assert_eq!(true,r);
}
#[cfg(test)]
#[test]

fn d() {
 //? allowed but * used
let r = qndr::numbers_with_symbols(&String::from("*ab5432cdews"),&String::from("?"));
    assert_eq!(false,r);
}
#[cfg(test)]
#[test]

fn e() {
 //abc allowed as symbols
let r = qndr::numbers_with_symbols(&String::from("12345abc"),&String::from("abc"));
assert_eq!(true,r);
}
#[cfg(test)]
#[test]

fn f() {

let r = qndr::numbers_with_symbols(&String::from("_1234"),&String::from("_a"));
assert_eq!(true,r);
}
#[test]
fn g() {

let r = qndr::numbers_with_symbols(&String::from("!@#$%^&*()_+0123456789"),&String::from("!@#$%^&*()_+"));
assert_eq!(true,r);
}
#[test]
fn h() {

let r = qndr::numbers_with_symbols(&String::from("`~!@#$%^&*()_+abcdews"),&String::from("?"));
    assert_eq!(false,r);
}

