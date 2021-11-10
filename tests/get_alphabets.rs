use qndr;

#[cfg(test)]
#[test]
fn get_alphabets_test() {
let a = qndr::get_alphabets(&String::from("1234567890"));
    assert_eq!("",a);

    let b = qndr::get_alphabets(&String::from("abc!dews"));
    assert_eq!("abcdews",b);

    let c = qndr::get_alphabets(&String::from("34dews"));
    assert_eq!("dews",c);

    let d = qndr::get_alphabets(&String::from("*ab5432cdews"));
    assert_eq!("abcdews",d);

    let e = qndr::get_alphabets(&String::from(" "));
assert_eq!("",e);

let f = qndr::get_alphabets(&String::from("_1234"));
assert_eq!("",f);

let g = qndr::get_alphabets(&String::from("abcdefghijlkmnopqrstuvwxyzABCDEFGHIJKLMNOPQRESTUVWXYZ"));
    assert_eq!("abcdefghijlkmnopqrstuvwxyzABCDEFGHIJKLMNOPQRESTUVWXYZ",g);

let h = qndr::get_alphabets(&String::from("5`~!@#$9%^&*(jjj)_d+f"));
    assert_eq!("jjjdf",h);
}

