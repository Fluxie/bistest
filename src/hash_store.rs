
use integer_set;
use std::collections::HashSet;

pub struct HashStore {    
    members: HashSet<i32>     
    
}

impl HashStore {
    
    pub fn new(         
        data: & integer_set::IntegerSet
    )  -> HashStore{        
        
        let mut store: HashSet< i32 > = HashSet::with_capacity( data.members.len() );
        for m in &data.members {
            store.insert( *m );
        }
        return HashStore { members:  store };    
    }
}

impl integer_set::SetStore for HashStore {    
    
    fn find(
        &self,
        value: &i32
    ) -> bool {
                
        let found = self.members.contains( value );
        return found;
    }
    
}