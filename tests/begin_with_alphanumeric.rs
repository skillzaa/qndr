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
let r = a.begin_with_alphanumeric(&String::from("a1c"));
    assert_eq!(true,r.unwrap());
}
#[cfg(test)]
#[test]

fn b() {
let a = Abc::new();
let r = a.begin_with_alphanumeric(&String::from("TgtvZ"));
assert_eq!(true,r.unwrap());

}
#[cfg(test)]
#[test]

fn c() {
let a = Abc::new();
let r = a.begin_with_alphanumeric(&String::from("4$#Dsi"));
assert_eq!(true,r.unwrap());

}
#[cfg(test)]
#[test]

fn d() {
let a = Abc::new();
let r = a.begin_with_alphanumeric(&String::from(" Q432cdews "));
    assert_eq!(None,r);
}
#[cfg(test)]
#[test]

fn e() {
let a = Abc::new();
let r = a.begin_with_alphanumeric(&String::from(" "));
assert_eq!(None,r);}
#[cfg(test)]
#[test]

fn f() {
let a = Abc::new();
let r = a.begin_with_alphanumeric(&String::from("_1234?"));
assert_eq!(None,r);}
#[test]
fn g() {
let a = Abc::new();
let r = a.begin_with_alphanumeric(&String::from("!1*"));
    assert_eq!(None,r);
}
#[test]
fn h() {
let a = Abc::new(); //space
let r = a.begin_with_alphanumeric(&String::from(" 5`~!@#$9%^&*()_+abcde6 "));
assert_eq!(None,r);
}

