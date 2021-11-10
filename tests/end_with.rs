use qndr;


#[cfg(test)]
#[test]
fn a() {

let r = qndr::end_with(&String::from("a"),
'a');
    assert_eq!(true,r);
}
#[cfg(test)]
#[test]

fn b() {

let r = qndr::end_with(&String::from("lgtv5"),'5');
assert_eq!(true,r);

}
#[cfg(test)]
#[test]

fn c() {

let r = qndr::end_with(&String::from("Z4$#DsZ"),'Z');
assert_eq!(true,r);

}
#[cfg(test)]
#[test]

fn d() {

let r = qndr::end_with(&String::from("Q432cdews "),'q');
    assert_eq!(false,r);
}
#[cfg(test)]
#[test]

fn e() {

let r = qndr::end_with(&String::from("ffffZ"),'z');
assert_eq!(false,r);}
#[cfg(test)]
#[test]

fn f() {

let r = qndr::end_with(&String::from("f"),'F');
assert_eq!(false,r);}
#[test]
fn g() {

let r = qndr::end_with(&String::from("!1"),' ');
    assert_eq!(false,r);
}
#[test]
fn h() {
 //space
let r = qndr::end_with(&String::from(" 5`~!@#$9%^&*()_+abcde6 "),'5');
assert_eq!(false,r);
}

