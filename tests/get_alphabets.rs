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
fn get_alphabets_test() {
let my_a = Abc::new();
let a = my_a.get_alphabets(&String::from("1234567890"));
    assert_eq!("",a);

    let b = my_a.get_alphabets(&String::from("abc!dews"));
    assert_eq!("abcdews",b);

    let c = my_a.get_alphabets(&String::from("34dews"));
    assert_eq!("dews",c);

    let d = my_a.get_alphabets(&String::from("*ab5432cdews"));
    assert_eq!("abcdews",d);

    let e = my_a.get_alphabets(&String::from(" "));
assert_eq!("",e);

let f = my_a.get_alphabets(&String::from("_1234"));
assert_eq!("",f);

let g = my_a.get_alphabets(&String::from("abcdefghijlkmnopqrstuvwxyzABCDEFGHIJKLMNOPQRESTUVWXYZ"));
    assert_eq!("abcdefghijlkmnopqrstuvwxyzABCDEFGHIJKLMNOPQRESTUVWXYZ",g);

let h = my_a.get_alphabets(&String::from("5`~!@#$9%^&*(jjj)_d+f"));
    assert_eq!("jjjdf",h);
}

