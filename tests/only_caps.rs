use qndr;


#[cfg(test)]
#[test]
fn a() {

let r = qndr::only_caps(&String::from("abcdefghijklmnopqrstuvwxyz"));
    assert_eq!(false,r);
}
#[cfg(test)]
#[test]

fn b() {

let r = qndr::only_caps(&String::from("0123456789abc!dew!@#$%^&*()_+s"));
assert_eq!(false,r);
}
#[cfg(test)]
#[test]

fn c() {

let r = qndr::only_caps(&String::from(" "));
    assert_eq!(false,r);
}
#[cfg(test)]
#[test]

fn d() {

let r = qndr::only_caps(&String::from(""));
assert_eq!(true,r);
}
#[cfg(test)]
#[test]

fn e() {

let r = qndr::only_caps(&String::from("anvF"));
assert_eq!(false,r);
}