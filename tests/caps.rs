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
let r = a.no_caps(&String::from("abcdefghijklmnopqrstuvwxyz"));
    assert_eq!(Some(true),r);
}
#[cfg(test)]
#[test]

fn b() {
let a = Abc::new();
let r = a.no_caps(&String::from("0123456789abc!dew!@#$%^&*()_+s"));
assert_eq!(Some(true),r);
}
#[cfg(test)]
#[test]

fn c() {
let a = Abc::new();
let r = a.no_caps(&String::from(" "));
    assert_eq!(Some(true),r);
}
#[cfg(test)]
#[test]

fn d() {
let a = Abc::new();
let r = a.no_caps(&String::from(""));
assert_eq!(Some(true),r);
}
#[cfg(test)]
#[test]

fn e() {
let a = Abc::new();
let r = a.no_caps(&String::from("anvF"));
assert_eq!(None,r);
}