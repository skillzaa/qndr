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
let r = a.numbers_with_symbols(&String::from("?1234567890"),&String::from("?"));
    assert_eq!(true,r.unwrap());
}
#[cfg(test)]
#[test]

fn b() {
    // !@# are allowed but # not used  --its false since abc etc are used but not allowed  
let a = Abc::new();
let r = a.numbers_with_symbols(&String::from("abc!dew@s"),&String::from("!@#"));
assert_eq!(None,r);
}
#[cfg(test)]
#[test]

fn c() {
let a = Abc::new(); //no symbols allowed but dews allowed and used
let r = a.numbers_with_symbols(&String::from("34dews"),&String::from("dews"));
assert_eq!(true,r.unwrap());
}
#[cfg(test)]
#[test]

fn d() {
let a = Abc::new(); //? allowed but * used
let r = a.numbers_with_symbols(&String::from("*ab5432cdews"),&String::from("?"));
    assert_eq!(None,r);
}
#[cfg(test)]
#[test]

fn e() {
let a = Abc::new(); //abc allowed as symbols
let r = a.numbers_with_symbols(&String::from("12345abc"),&String::from("abc"));
assert_eq!(true,r.unwrap());
}
#[cfg(test)]
#[test]

fn f() {
let a = Abc::new();
let r = a.numbers_with_symbols(&String::from("_1234"),&String::from("_a"));
assert_eq!(true,r.unwrap());
}
#[test]
fn g() {
let a = Abc::new();
let r = a.numbers_with_symbols(&String::from("!@#$%^&*()_+0123456789"),&String::from("!@#$%^&*()_+"));
assert_eq!(true,r.unwrap());
}
#[test]
fn h() {
let a = Abc::new();
let r = a.numbers_with_symbols(&String::from("`~!@#$%^&*()_+abcdews"),&String::from("?"));
    assert_eq!(None,r);
}

