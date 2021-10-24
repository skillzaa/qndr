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
let r = a.get_non_numbers(&String::from("abcdefghijlkmnopqrstuvwxyzABCDEFGHIJKLMNOPQRESTUVWXYZ1234567890"));
    assert_eq!("abcdefghijlkmnopqrstuvwxyzABCDEFGHIJKLMNOPQRESTUVWXYZ",r);
}
#[cfg(test)]
#[test]

fn b() {
let a = Abc::new();
let r = a.get_non_numbers(&String::from("abc!de3w7s"));
 assert_eq!("abc!dews",r);
}
#[cfg(test)]
#[test]

fn c() {
let a = Abc::new();
let r = a.get_non_numbers(&String::from("!@#$%^&*()34dews"));
    assert_eq!("!@#$%^&*()dews",r);
}
#[cfg(test)]
#[test]

fn d() {
let a = Abc::new();
let r = a.get_non_numbers(&String::from(" "));
    assert_eq!(" ",r); // the " " is a non alphabet
}
#[cfg(test)]
#[test]

fn e() {
let a = Abc::new();
let r = a.get_non_numbers(&String::from(""));
assert_eq!("",r);
}
#[cfg(test)]
#[test]

fn f() {
let a = Abc::new();
let r = a.get_non_numbers(&String::from("_1234ccc"));
assert_eq!("_ccc",r);
}
#[test]
fn g() {
let a = Abc::new();
let r = a.get_non_numbers(&String::from("!q2"));
    assert_eq!("!q",r);
}
#[test]
fn h() {
let a = Abc::new();
let r = a.get_non_numbers(&String::from("5`~!@#$9%^&*()_+abc"));
    assert_eq!("`~!@#$%^&*()_+abc",r);
}

