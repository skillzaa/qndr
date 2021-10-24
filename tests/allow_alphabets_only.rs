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
let r = a.allow_alphabets_only(&String::from("abcdews"));
    assert_eq!(true,r.unwrap());
}
#[cfg(test)]
#[test]

fn b() {
let a = Abc::new();
let r = a.allow_alphabets_only(&String::from("abc!dews"));
// println!("{:?}",r);    
 assert_eq!(None,r);
}
#[cfg(test)]
#[test]

fn c() {
let a = Abc::new();
let r = a.allow_alphabets_only(&String::from("34dews"));
    assert_eq!(None,r);
}
#[cfg(test)]
#[test]

fn d() {
let a = Abc::new();
let r = a.allow_alphabets_only(&String::from("*abcdews"));
    assert_eq!(None,r);
}
#[cfg(test)]
#[test]

fn e() {
let a = Abc::new();
let r = a.allow_alphabets_only(&String::from(" "));
assert_eq!(None,r);
}
#[cfg(test)]
#[test]

fn f() {
let a = Abc::new();
let r = a.allow_alphabets_only(&String::from("0123456789"));
assert_eq!(None,r);
}
#[test]
fn g() {
let a = Abc::new();
let r = a.allow_alphabets_only(&String::from("abcdefghijlkmnopqrstuvwxyzABCDEFGHIJKLMNOPQRESTUVWXYZ"));
    assert_eq!(true,r.unwrap());
}
#[test]
fn h() {
let a = Abc::new();
let r = a.allow_alphabets_only(&String::from("`~!@#$%^&*()_+abcdews"));
    assert_eq!(None,r);
}

