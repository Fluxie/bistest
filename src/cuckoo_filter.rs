
extern crate cuckoofilter;

use integer_set;
use std::hash::{SipHasher, Hasher, Hash};

pub struct CuckooFilterStore {
    probable_members: cuckoofilter::CuckooFilter< SipHasher >,
    probable_non_members: cuckoofilter::CuckooFilter< SipHasher >,
    members: Vec< i32 >

}

impl<'a> CuckooFilterStore {

    pub fn new(
        data: &'a integer_set::IntegerSet
    )  -> CuckooFilterStore{

        // Filter to members.
        let mut  positive_filter  = cuckoofilter::CuckooFilter::with_capacity( data.members.len() + 1000  );
        for m in &data.members {
            positive_filter.add( &m );
        }

        // Filter for non members.
        let mut negative_filter = cuckoofilter::CuckooFilter::with_capacity( data.non_members.len() + 1000 );
        for nm in &data.non_members {
            negative_filter.add( &nm );
        }

        // Store the vector as sorted.
        let mut sorted = data.members.to_vec();
        sorted.sort();

        return CuckooFilterStore { probable_members: positive_filter, probable_non_members: negative_filter, members: sorted  };
    }
}

impl<'a> integer_set::SetStore for CuckooFilterStore {

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
        match self.members.binary_search( value ) {
            Ok(_) => return true,
            Err(_) => return false
        }
    }

}