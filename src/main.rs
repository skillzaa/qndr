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
a.ends_with_number(&"zdews5".to_string());        
}