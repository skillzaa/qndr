use qndr;


#[cfg(test)]
#[test]
fn a() {

let r = qndr::no_caps(&String::from("abcdefghijklmnopqrstuvwxyz"));
    assert_eq!(true,r);
}
#[cfg(test)]
#[test]

fn b() {

let r = qndr::no_caps(&String::from("0123456789abc!dew!@#$%^&*()_+s"));
assert_eq!(true,r);
}
#[cfg(test)]
#[test]

fn c() {

let r = qndr::no_caps(&String::from(" "));
    assert_eq!(true,r);
}
#[cfg(test)]
#[test]

fn d() {

let r = qndr::no_caps(&String::from(""));
assert_eq!(true,r);
}
#[cfg(test)]
#[test]

fn e() {

let r = qndr::no_caps(&String::from("anvF"));
assert_eq!(false,r);
}