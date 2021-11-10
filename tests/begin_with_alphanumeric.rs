use qndr;


#[cfg(test)]
#[test]
fn a() {

let r = qndr::begin_with_alphanumeric(&String::from("a1c"));
    assert_eq!(true,r);
}
#[cfg(test)]
#[test]

fn b() {

let r = qndr::begin_with_alphanumeric(&String::from("TgtvZ"));
assert_eq!(true,r);

}
#[cfg(test)]
#[test]

fn c() {

let r = qndr::begin_with_alphanumeric(&String::from("4$#Dsi"));
assert_eq!(true,r);

}
#[cfg(test)]
#[test]

fn d() {

let r = qndr::begin_with_alphanumeric(&String::from(" Q432cdews "));
    assert_eq!(false,r);
}
#[cfg(test)]
#[test]

fn e() {

let r = qndr::begin_with_alphanumeric(&String::from(" "));
assert_eq!(false,r);}
#[cfg(test)]
#[test]

fn f() {

let r = qndr::begin_with_alphanumeric(&String::from("_1234?"));
assert_eq!(false,r);}
#[test]
fn g() {

let r = qndr::begin_with_alphanumeric(&String::from("!1*"));
    assert_eq!(false,r);
}
#[test]
fn h() {
 //space
let r = qndr::begin_with_alphanumeric(&String::from(" 5`~!@#$9%^&*()_+abcde6 "));
assert_eq!(false,r);
}

