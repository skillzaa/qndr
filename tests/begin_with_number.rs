use qndr;


#[cfg(test)]
#[test]
fn a() {

let r = qndr::begin_with_number(&String::from("1"));
    assert_eq!(true,r);
}
#[cfg(test)]
#[test]

fn b() {

let r = qndr::begin_with_number(&String::from("5lgtv!"));
assert_eq!(true,r);

}
#[cfg(test)]
#[test]

fn c() {

let r = qndr::begin_with_number(&String::from("34$#Dsa"));
assert_eq!(true,r);

}
#[cfg(test)]
#[test]

fn d() {

let r = qndr::begin_with_number(&String::from(" 5432cdews"));
    assert_eq!(false,r);
}
#[cfg(test)]
#[test]

fn e() {

let r = qndr::begin_with_number(&String::from(" "));
assert_eq!(false,r);}
#[cfg(test)]
#[test]

fn f() {

let r = qndr::begin_with_number(&String::from("_1234"));
assert_eq!(false,r);}
#[test]
fn g() {

let r = qndr::begin_with_number(&String::from("!1"));
    assert_eq!(false,r);
}
#[test]
fn h() {

let r = qndr::begin_with_number(&String::from(".5`~!@#$9%^&*()_+abcdews"));
assert_eq!(false,r);
}

