
use integer_set;
use ro_hash_set;

pub struct RoHashStore {    
    members: ro_hash_set::RoHashSet
    
}

impl RoHashStore {
    
    pub fn new(         
        data: & integer_set::IntegerSet
    )  -> RoHashStore{        
        
        let members = ro_hash_set::RoHashSet::new( &data.members, data.size as i32 );        
        return RoHashStore { members:  members };    
    }
}

impl integer_set::SetStore for RoHashStore {    
    
    fn find(
        &self,
        value: &i32
    ) -> bool {
                
        let found = self.members.contains( value );
        return found;
    }
    
}