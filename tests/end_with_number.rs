use qndr;


#[cfg(test)]
#[test]
fn a() {

let r = qndr::end_with_number(&String::from("1"));
    assert_eq!(true,r);
}
#[cfg(test)]
#[test]

fn b() {

let r = qndr::end_with_number(&String::from("5lgtv5"));
assert_eq!(true,r);

}
#[cfg(test)]
#[test]

fn c() {

let r = qndr::end_with_number(&String::from("w4$#Ds5"));
assert_eq!(true,r);

}
#[cfg(test)]
#[test]

fn d() {

let r = qndr::end_with_number(&String::from(" 5432cdews "));
    assert_eq!(false,r);
}
#[cfg(test)]
#[test]

fn e() {

let r = qndr::end_with_number(&String::from(" "));
assert_eq!(false,r);}
#[cfg(test)]
#[test]

fn f() {

let r = qndr::end_with_number(&String::from("_1234?"));
assert_eq!(false,r);}
#[test]
fn g() {

let r = qndr::end_with_number(&String::from("!1!"));
    assert_eq!(false,r);
}
#[test]
fn h() {
 //space
let r = qndr::end_with_number(&String::from(".5`~!@#$9%^&*()_+abcde6 "));
assert_eq!(false,r);
}

