use qndr::Qndr;
#[derive(Debug)]
struct Abc {
    name:String,
}

impl Abc {
    fn new()->Abc{
        Abc {
            name: String::from("bbb"),
        }
    }
}
impl Qndr for Abc {}

#[cfg(test)]
#[test]
fn a() {
let a = Abc::new();
let r = a.begin_with(&String::from("a1"),
'a');
    assert_eq!(true,r.unwrap());
}
#[cfg(test)]
#[test]

fn b() {
let a = Abc::new();
let r = a.begin_with(&String::from("lgtv5"),'l');
assert_eq!(true,r.unwrap());

}
#[cfg(test)]
#[test]

fn c() {
let a = Abc::new();
let r = a.begin_with(&String::from("Z4$#Ds5"),'Z');
assert_eq!(true,r.unwrap());

}
#[cfg(test)]
#[test]

fn d() {
let a = Abc::new();
let r = a.begin_with(&String::from("Q432cdews "),'q');
    assert_eq!(None,r);
}
#[cfg(test)]
#[test]

fn e() {
let a = Abc::new();
let r = a.begin_with(&String::from("Z"),'z');
assert_eq!(None,r);}
#[cfg(test)]
#[test]

fn f() {
let a = Abc::new();
let r = a.begin_with(&String::from("_1234?"),'f');
assert_eq!(None,r);}
#[test]
fn g() {
let a = Abc::new();
let r = a.begin_with(&String::from("!1!"),' ');
    assert_eq!(None,r);
}
#[test]
fn h() {
let a = Abc::new(); //space
let r = a.begin_with(&String::from(" 5`~!@#$9%^&*()_+abcde6 "),'5');
assert_eq!(None,r);
}

