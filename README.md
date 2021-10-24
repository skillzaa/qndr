# QNDR - Quick and Dirty Regex
---
>Simple and useful methods for simple String / Regex operations.
---
**QNDR** exports just one trait.  Once this trait is implemented by some struct it gets (25 something) simple and useful methods for simple regex operations of strings.
---
## Example
*please see the documentation for details of methods*
```rust
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

// This is all the magic you need!!!!
impl Qndr for Abc {}

fn main(){
let a = Abc::new();
let result_a = a.begin_with_alphanumeric(&String::from("!!!!Hayyy"));
assert_eq!(None,r);

let result_b = a.begin_with_alphabet(&String::from("_1234?"));
assert_eq!(None,r);

let r = a.allow_alphabets_only(&String::from("0123456789"));
assert_eq!(None,r);

let r = a.allow_alphanumeric_only(&String::from(" "));
assert_eq!(None,r);


}
```

##### 23-oct-2021
> This is very simple but useful crate. Main purpose of this package is for teaching and educational purposes. 
> All the documents and tests are well maintained. Incase of any problem please contact me and I will take care of it. 
> You are encouraged to look at the code and point out any problems or errors. **I will add features only if they are very valid**.
---
