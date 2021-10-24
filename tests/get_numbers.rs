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
let r = a.get_numbers(&String::from("1234567890"));
    assert_eq!("1234567890",r);
}
#[cfg(test)]
#[test]

fn b() {
let a = Abc::new();
let r = a.get_numbers(&String::from("abc!dews"));
// println!("{:?}",r);    
 assert_eq!("",r);
}
#[cfg(test)]
#[test]

fn c() {
let a = Abc::new();
let r = a.get_numbers(&String::from("34dews"));
    assert_eq!("34",r);
}
#[cfg(test)]
#[test]

fn d() {
let a = Abc::new();
let r = a.get_numbers(&String::from("*ab5432cdews"));
    assert_eq!("5432",r);
}
#[cfg(test)]
#[test]

fn e() {
let a = Abc::new();
let r = a.get_numbers(&String::from(" "));
assert_eq!("",r);
}
#[cfg(test)]
#[test]

fn f() {
let a = Abc::new();
let r = a.get_numbers(&String::from("_1234"));
assert_eq!("1234",r);
}
#[test]
fn g() {
let a = Abc::new();
let r = a.get_numbers(&String::from("abcdefghijlkmnopqrstuvwxyzABCDEFGHIJKLMNOPQRESTUVWXYZ"));
    assert_eq!("",r);
}
#[test]
fn h() {
let a = Abc::new();
let r = a.get_numbers(&String::from("5`~!@#$9%^&*()_+abcdews"));
    assert_eq!("59",r);
}

