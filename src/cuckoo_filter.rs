
extern crate cuckoofilter;

use integer_set;
use std::hash::{SipHasher, Hasher, Hash};

pub struct CuckooFilterStore<'a> {    
    probable_members: cuckoofilter::CuckooFilter< SipHasher >,
    probable_non_members: cuckoofilter::CuckooFilter< SipHasher >,
    members: &'a Vec< i32 >
    
}

impl<'a> CuckooFilterStore<'a> {
    
    pub fn new(         
        data: &'a integer_set::IntegerSet
    )  -> CuckooFilterStore{        
                     
        // Filter to members.
        let mut  positive_filter  = cuckoofilter::CuckooFilter::new();
        for m in &data.members {
            positive_filter.add( &m );
        }
        
        // Filter for non members.
        let mut negative_filter = cuckoofilter::CuckooFilter::new();
        for nm in &data.non_members {
            negative_filter.add( &nm );
        }
        
        return CuckooFilterStore { probable_members: positive_filter, probable_non_members: negative_filter, members: &data.members };
    }
}

impl<'a> integer_set::SetStore for CuckooFilterStore<'a> {    
    
    fn find(
        &self,
        value: &i32
    ) -> bool {
        
        // Not a amember?
        let probably_member = self.probable_members.contains( &value );
        if ! probably_member { return false; }
        
        // A member?
        let probably_non_member = self.probable_non_members.contains( &value );
        if ! probably_non_member { return true; }
        
        // Full check is required
        println!( "full check" );
        for m in self.members {
            if m == value {
                return true;
            }
        }
        return false;
    }
    
}