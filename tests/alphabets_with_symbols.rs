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
let r = a.alphabets_with_symbols(&String::from("?abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"),&String::from("?"));
    assert_eq!(true,r.unwrap());
}
#[cfg(test)]
#[test]

fn b() {  
let a = Abc::new();
let r = a.alphabets_with_symbols(&String::from("abc!dew@s"),&String::from("!@#"));
assert_eq!(true,r.unwrap());
}
#[cfg(test)]
#[test]

fn c() {
let a = Abc::new(); //no symbols allowed but dews allowed and used
let r = a.alphabets_with_symbols(&String::from("dews!@#$%^&*()"),&String::from("!@#$%^&*()"));
assert_eq!(true,r.unwrap());
}
#[cfg(test)]
#[test]

fn d() {
let a = Abc::new(); //? allowed but * used
let r = a.alphabets_with_symbols(&String::from("*ab5432cdews"),&String::from("?"));
    assert_eq!(None,r);
}
#[cfg(test)]
#[test]

fn e() {
let a = Abc::new(); //abc allowed as symbols
let r = a.alphabets_with_symbols(&String::from("abc*****"),&String::from("**")); //lets see if repeating in pattern cause prob
assert_eq!(true,r.unwrap());
}
#[cfg(test)]
#[test]

fn f() {
let a = Abc::new();
let r = a.alphabets_with_symbols(&String::from("_abcd"),&String::from("_"));
assert_eq!(true,r.unwrap());
}
#[test]
fn g() {
let a = Abc::new();
let r = a.alphabets_with_symbols(&String::from("!@#$%^&*()_+ABCDEF"),&String::from("!@#$%^&*()_+"));
assert_eq!(true,r.unwrap());
}
#[test]
fn h() {
let a = Abc::new();
let r = a.alphabets_with_symbols(&String::from("`~!@#$%^&*()_+abcdews"),&String::from("`~!@#$%^&*()_+"));
    assert_eq!(true,r.unwrap());
}
