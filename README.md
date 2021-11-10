# QNDR - Quick and Dirty Regex
---
>Simple and useful methods for simple Regex operations.
---
> [Documentation](https://docs.rs/qndr)
---
**QNDR** exports (25 something) simple and useful methods for simple regex operations.
This library has no dependencies.
---
## Example
*please see the documentation for details of methods*
```rust
use qndr;

fn main(){
let result_a = qndr::begin_with_alphanumeric(&String::from("!!!!Hayyy"));
assert_eq!(false,result_a);

let result_b = qndr::begin_with_alphabet(&String::from("_1234?"));
assert_eq!(false,result_b);

let result_c = qndr::allow_alphabets_only(&String::from("0123456789"));
assert_eq!(false,result_c);

let result_d = qndr::allow_alphanumeric_only(&String::from(" "));
assert_eq!(false,result_d);

println!("All operations completed...");
}

```

##### 23-oct-2021
> This is very simple but useful crate. Main purpose of this library is for teaching and educational purposes. 
> All the documents and tests are well maintained. Incase of any problem please contact me and I will take care of it. 
> You are encouraged to look at the code and point out any problems or errors. **I will add features only if they are very valid**.
---
