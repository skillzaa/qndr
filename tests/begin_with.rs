use qndr;


#[cfg(test)]
#[test]
fn a() {

let r = qndr::begin_with(&String::from("a1"),
'a');
    assert_eq!(true,r);
}
#[cfg(test)]
#[test]

fn b() {

let r = qndr::begin_with(&String::from("lgtv5"),'l');
assert_eq!(true,r);

}
#[cfg(test)]
#[test]

fn c() {

let r = qndr::begin_with(&String::from("Z4$#Ds5"),'Z');
assert_eq!(true,r);

}
#[cfg(test)]
#[test]

fn d() {

let r = qndr::begin_with(&String::from("Q432cdews "),'q');
    assert_eq!(false,r);
}
#[cfg(test)]
#[test]

fn e() {

let r = qndr::begin_with(&String::from("Z"),'z');
assert_eq!(false,r);}
#[cfg(test)]
#[test]

fn f() {

let r = qndr::begin_with(&String::from("_1234?"),'f');
assert_eq!(false,r);}
#[test]
fn g() {

let r = qndr::begin_with(&String::from("!1!"),' ');
    assert_eq!(false,r);
}
#[test]
fn h() {
 //space
let r = qndr::begin_with(&String::from(" 5`~!@#$9%^&*()_+abcde6 "),'5');
assert_eq!(false,r);
}

