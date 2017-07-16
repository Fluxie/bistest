
use integer_set;

pub struct SortedStore {    
    members: Vec<i32>     
    
}

impl SortedStore {
    
    pub fn new(         
        data: & integer_set::IntegerSet
    )  -> SortedStore{        
        
        // Store the vector as sorted.
        let mut sorted = data.members.to_vec();
        sorted.sort();
        return SortedStore { members:  sorted };    
    }
}

impl integer_set::SetStore for SortedStore {    
    
    fn find(
        &self,
        value: &i32
    ) -> bool {
                
        match self.members.binary_search( value ) {
            Ok(_) => return true,
            Err(_) => return false
        }
    }
    
}