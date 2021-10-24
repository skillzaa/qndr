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
let r = a.end_with(&String::from("a"),
'a');
    assert_eq!(true,r.unwrap());
}
#[cfg(test)]
#[test]

fn b() {
let a = Abc::new();
let r = a.end_with(&String::from("lgtv5"),'5');
assert_eq!(true,r.unwrap());

}
#[cfg(test)]
#[test]

fn c() {
let a = Abc::new();
let r = a.end_with(&String::from("Z4$#DsZ"),'Z');
assert_eq!(true,r.unwrap());

}
#[cfg(test)]
#[test]

fn d() {
let a = Abc::new();
let r = a.end_with(&String::from("Q432cdews "),'q');
    assert_eq!(None,r);
}
#[cfg(test)]
#[test]

fn e() {
let a = Abc::new();
let r = a.end_with(&String::from("ffffZ"),'z');
assert_eq!(None,r);}
#[cfg(test)]
#[test]

fn f() {
let a = Abc::new();
let r = a.end_with(&String::from("f"),'F');
assert_eq!(None,r);}
#[test]
fn g() {
let a = Abc::new();
let r = a.end_with(&String::from("!1"),' ');
    assert_eq!(None,r);
}
#[test]
fn h() {
let a = Abc::new(); //space
let r = a.end_with(&String::from(" 5`~!@#$9%^&*()_+abcde6 "),'5');
assert_eq!(None,r);
}

