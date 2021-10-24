
pub trait Qndr {
    //=============// Allow //==================
    fn allow_numbers_only(&self , sample:&String )->Option<bool>{
        for c in sample.chars() { 
            let r = c.is_numeric();
            match r {
                true=> {},
                false=> return None,
            }
        }
        Some(true)
    }
    fn allow_alphanumeric_only(&self , sample:&String )->Option<bool>{
        for c in sample.chars() { 
            let r = c.is_alphanumeric();
            match r {
                true=> {},
                false=> return None,
            }
        }
        Some(true)
    }
    fn allow_alphabets_only(&self , sample:&String )->Option<bool>{
        for c in sample.chars() { 
            let r = c.is_alphabetic();
            match r {
                true=> {},
                false=> return None,
            }
        }
        Some(true)    
    }

    //=============// Get //==================
    fn get_alphabets(&self,sample:&String )->String{
        let mut alphabets = String::from(""); 
        for c in sample.chars() { 
            let r = c.is_alphabetic();
            match r {
                true=> {alphabets.push(c)},
                false=> {},
            }
        }
        alphabets    
    }
    fn get_numbers(&self,sample: &String)->String{
        let mut numbers = String::from(""); 
        for c in sample.chars() { 
            let r = c.is_numeric();
            match r {
                true=> {numbers.push(c)},
                false=> {},
            }
        }
        numbers    
    }
    fn get_alphanumeric(&self,sample:&String )->String{
        let mut alphanumeric = String::from(""); 
        for c in sample.chars() { 
            let r = c.is_alphanumeric();
            match r {
                true=> {alphanumeric.push(c)},
                false=> {},
            }
        }
        alphanumeric    
    }
    
    //=============// Get Non //==================    
    fn get_non_alphabets(&self,sample:&String )->String{
        let mut non_alphabets = String::from(""); 
        for c in sample.chars() { 
            let r = !c.is_alphabetic(); //note !
            match r {
                true=> {non_alphabets.push(c)},
                false=> {},
            }
        }
        non_alphabets    
    }
    fn get_non_numbers(&self,sample:&String )->String{
        let mut numbers = String::from(""); 
        for c in sample.chars() { 
            let r = !c.is_numeric();
            match r {
                true=> {numbers.push(c)},
                false=> {},
            }
        }
        numbers    
    }
    fn get_non_alphanumeric(&self,sample:&String )->String{
        let mut non_alphanumeric = String::from(""); 
        for c in sample.chars() { 
            let r = !c.is_alphanumeric();
            match r {
                true=> {non_alphanumeric.push(c)},
                false=> {},
            }
        }
        non_alphanumeric    
    }
    
    //=============// With Symbols //==================    
    fn numbers_with_symbols(&self,sample:&String,allowed_symbols:&String)->Option<bool>{
      let just_numbers = self.get_numbers(&sample);
      let just_symbols = self.remove_chars(&sample , &just_numbers);
      let result = self.check_string_for_allowed_chars(&just_symbols,&allowed_symbols);
      result
    }

    fn alphabets_with_symbols(&self , sample:&String,allowed_symbols:&String )->Option<bool>{
        let non_alpha = self.get_non_alphabets(&sample);
        let result = self.check_string_for_allowed_chars
        (&non_alpha,&allowed_symbols);
        result
    }
    fn alphanumeric_with_symbols(&self , sample:&String ,allowed_symbols:&String  )->Option<bool>{
        let non_alpha_num = self.get_non_alphanumeric(&sample);
        let result = self.check_string_for_allowed_chars
        (&non_alpha_num,&allowed_symbols);
        result
    }  
    //=============// Caps //==================    
    ///This function will return None even if a single char is found
    /// not of a lower case alphabet.
    /// Even a single symbols like ?, ! , & etc will trigger a None 
    fn no_caps(&self , sample:&String )->Option<bool>{
        for i in sample.chars(){
            if i.is_ascii_uppercase() {return None}
        }
        Some(true)
    }
    /// The data presented to this function should be **alphabatic**
    /// and in all caps. If any char found that is not a **capital alphabet** the function will return None.    
    ///This function looks for any non capital alphabet in the sample.
    ///Even if it finds one non capital char it will return None.
    /// The function will return None unless the sample string is entirely comprised if *ABCDEFGHIJKLMNOPQRSTUVWXYZ*

    fn only_caps(&self , sample:&String )->Option<bool>{
        for i in sample.chars(){
            if i.is_ascii_lowercase() {return None}
        }
        Some(true)
    }

    //=============// Begins and Ends //================== 
    
    fn begin_with(&self , sample:&String,begin_char:char )->Option<bool>{
        let first:char = sample.chars().nth(0).unwrap();
        if first == begin_char { return Some(true)}else{return None}
    }
    fn end_with(&self , sample:&String,end_char:char )->Option<bool>{
        let last:char = sample.chars().rev().nth(0).unwrap();
        if last == end_char {return Some(true)}else{return None}
    }
    fn begin_with_number(&self , sample:&String )->Option<bool>{
        let first:char = sample.chars().nth(0).unwrap();
        if first.is_numeric() { return Some(true) } else { return None };
    }
    fn end_with_number(&self , sample:&String )->Option<bool>{
        let last:char = sample.chars().rev().nth(0).unwrap();
        if last.is_numeric() { return Some(true) } else { return None };
    }
    fn begin_with_alphabet(&self , sample:&String )->Option<bool>{
        let first:char = sample.chars().nth(0).unwrap();
        if first.is_alphabetic() { return Some(true) } else { return None };
    }
    fn end_with_alphabet(&self , sample:&String )->Option<bool>{
        let last:char = sample.chars().rev().nth(0).unwrap();
        if last.is_alphabetic() { return Some(true) } else { return None };
    }
    fn begin_with_alphanumeric(&self , sample:&String )->Option<bool>{
        let first:char = sample.chars().nth(0).unwrap();
        if first.is_alphanumeric() { return Some(true) } else { return None };
    }
    fn end_with_alphanumeric(&self , sample:&String )->Option<bool>{
        let last:char = sample.chars().rev().nth(0).unwrap();
        if last.is_alphanumeric() { return Some(true) } else { return None };
    }
       
    //=============// Misc //================== 
    fn string_to_vec (&self,incomming:&String)->Vec<char>{
        let mut chars:Vec<char> = Vec::new();
            for i in incomming.chars() {
                chars.push(i);
            }
        chars
    }
    // fn is_valid_url(&self , sample:&String )->Option<bool>{
    //     todo!();
    // }
    fn check_string_for_allowed_chars(&self,data:&String,allowed_chars:&String)->Option<bool>{
        for i in data.chars(){
                match allowed_chars.contains(i){
                    true=> {},
                    false=> {return None}
                }
            }
        Some(true)
    }
    fn remove_chars(&self,large_string:&String,to_be_sub:&String)->String{
        let mut result:String = String::from("");
        for i in large_string.chars(){
            if !to_be_sub.contains(i){
                let j = i.clone();
                result.push(j); 
            }
        }
        result
    }

    
  

}//trait ends

