use qndr::Qndr;
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

fn main(){
let a = Abc::new();
let r = a.numbers_with_symbols(&String::from("abc!dew@s"),&String::from("!@#"));
        
}