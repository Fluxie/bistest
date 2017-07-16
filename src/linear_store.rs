
use integer_set;

pub struct LinearStore<'a> {    
    members: &'a Vec<i32>     
    
}

impl<'a> LinearStore<'a> {
    
    pub fn new(         
        data: &'a integer_set::IntegerSet
    )  -> LinearStore{        
        return LinearStore { members: &data.members };    
    }
}

impl<'a> integer_set::SetStore for LinearStore<'a> {    
    
    fn find(
        &self,
        value: &i32
    ) -> bool {
        
        for m in self.members {
            if m == value {
                return true;
            }
        }
        
        // The value was not found.
        return false;
    }
    
}