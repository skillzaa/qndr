
pub trait Qndr {
    
    fn alphabets_only(&self , sample:String )->bool{
        for c in sample.chars() { 
            let r = c.is_alphabetic();
            match r {
                true=> {},
                false=> return false,
            }
        }
    true    
    }
    fn numbers_only(&self , sample:String ){
        todo!();
    }
    fn alphanumeric_only(&self , sample:String ){
        todo!();
    }

    fn numbers_with_symbols(&self , sample:String ){
        todo!();
    }
    fn alphabets_with_symbols(&self , sample:String ){
        todo!();
    }
    fn alphanumeric_with_symbols(&self , sample:String ){
        todo!();
    }

    fn no_caps(&self , sample:String ){
        todo!();
    }
    fn only_caps(&self , sample:String ){
        todo!();
    }

    //-------------------------------
    fn begins_with(&self , sample:String ){
        todo!();
    }
    fn ends_with(&self , sample:String ){
        todo!();
    }
    fn begins_with_number(&self , sample:String ){
        todo!();
    }
    fn ends_with_number(&self , sample:String ){
        todo!();
    }
    fn begins_with_alphabet(&self , sample:String ){
        todo!();
    }
    fn ends_with_alphabet(&self , sample:String ){
        todo!();
    }
    fn begins_with_alphanumeric(&self , sample:String ){
        todo!();
    }
    fn ends_with_alphanumeric(&self , sample:String ){
        todo!();
    }
    fn begins_with_symbol(&self , sample:String ){
        todo!();
    }
    fn ends_with_symbol(&self , sample:String ){
        todo!();
    }
    //-------------------------------

    fn is_valid_url(&self , sample:String ){
        todo!();
    }

}//trait ends