
pub trait Qndr {
    //=============// Allow //==================
    /// This fn takes a string (by ref) and will check to see that  
    /// the string should only consist of numerical chars.
    /// The fn will return false even if one non-numeric char found
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
    /// This fn takes a string (by ref) and will check to see that  
    /// the string should only consist of alpha-numerical chars.
    /// The fn will return false even if one non-alpha-numerical char 
    /// found.    
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
     /// This fn takes a string (by ref) and will check to see that  
    /// the string should only consist of alphabets.
    /// The fn will return false even if one non-alphabetic char found
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
    /// This fn takes a string by ref and return (as a new string)all
    /// the alphabetic chars.
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
    /// This fn takes a string by ref and return (as a new string)all
    /// the numeric chars.
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
    /// This fn takes a string by ref and return (as a new string)all
    /// the alpha-numeric chars.
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
    /// This fn takes a string by ref and returns (as a new string) all
    /// the **non alphabets** chars .
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
    /// This fn takes a string by ref and returns (as a new string) 
    /// all the non *Numerical* chars .
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
    /// This fn takes a string by ref and returns (as a new string) 
    /// all the non *Alpha-Numeric* chars.
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
    /// We use this fn if we want to allow just numbers and few
    /// selected symbols e.g ! , @ etc. It takes a &String 
    /// (the sample string) and another &String with allowed symbols.
    /// It will return true only of the sample string has only
    /// numbers and the allowed symbols provided.
    fn numbers_with_symbols(&self,sample:&String,allowed_symbols:&String)->Option<bool>{
      let just_numbers = self.get_numbers(&sample);
      let just_symbols = self.remove_chars(&sample , &just_numbers);
      let result = self.check_string_for_allowed_chars(&just_symbols,&allowed_symbols);
      result
    }
    /// We use this fn if we want to allow just alphabets and few
    /// selected symbols e.g ! , @ etc. It takes a &String 
    /// (the sample string) and another &String with allowed symbols.
    /// It will return true only of the sample string has only
    /// alphabets and the allowed symbols provided.

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
    /// This function will return None even if a single Capital
    /// character is found in the provided sample &String.
    /// *Any character other than a capital case alphabet is OK* 
    fn no_caps(&self , sample:&String )->Option<bool>{
        for i in sample.chars(){
            if i.is_ascii_uppercase() {return None}
        }
        Some(true)
    }
    /// The data presented to this function should be **alphabatic**
    /// and in all caps. If any char found that is not a **capital
    /// alphabet** the function will return None.    
    ///This function looks for any non capital alphabet in the sample.
    ///Even if it finds one non capital char it will return None.
    /// The function will return None unless the sample string is 
    /// entirely comprising of *ABCDEFGHIJKLMNOPQRSTUVWXYZ*

    fn only_caps(&self , sample:&String )->Option<bool>{
        for i in sample.chars(){
            if !i.is_ascii_uppercase() {return None}
        }
        Some(true)
    }

    //=============// Begins and Ends //================== 
    
    /// This function checks if the sample string begins with the 
    /// provided character or not.
    fn begin_with(&self , sample:&String,begin_char:char )->Option<bool>{
        let first:char = sample.chars().nth(0).unwrap();
        if first == begin_char { return Some(true)}else{return None}
    }
    /// This function checks if the sample string ends with the 
    /// provided character or not.

    fn end_with(&self , sample:&String,end_char:char )->Option<bool>{
        let last:char = sample.chars().rev().nth(0).unwrap();
        if last == end_char {return Some(true)}else{return None}
    }
    /// This function checks if the sample string begins with a 
    /// number or not.

    fn begin_with_number(&self , sample:&String )->Option<bool>{
        let first:char = sample.chars().nth(0).unwrap();
        if first.is_numeric() { return Some(true) } else { return None };
    }
    /// This function checks if the sample string ends with a 
    /// number or not.

    fn end_with_number(&self , sample:&String )->Option<bool>{
        let last:char = sample.chars().rev().nth(0).unwrap();
        if last.is_numeric() { return Some(true) } else { return None };
    }
    /// This function checks if the sample string begins with an 
    /// alphabet or not.

    fn begin_with_alphabet(&self , sample:&String )->Option<bool>{
        let first:char = sample.chars().nth(0).unwrap();
        if first.is_alphabetic() { return Some(true) } else { return None };
    }

    /// This function checks if the sample string begins with an 
    /// alphabet or not.
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

