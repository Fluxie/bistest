
use integer_set;

pub struct VectorStore<'a> {    
    members: &'a Vec<i32>     
    
}

impl<'a> VectorStore<'a> {
    
    pub fn new(         
        data: &'a integer_set::IntegerSet
    )  -> VectorStore{        
        return VectorStore { members: &data.members };    
    }
}

impl<'a> integer_set::SetStore for VectorStore<'a> {    
    
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