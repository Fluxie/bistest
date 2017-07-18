#![allow(dead_code)]

extern crate rand;

use rand::Rng;
use std::collections::HashSet;

pub fn create(
    set_size: usize,
    member_count: usize
) -> IntegerSet {     
     
     // Prevent duplicates.
     let mut generated: HashSet< i32 > = HashSet::with_capacity( member_count );
     
     // Generate members.
     let mut members: Vec<i32>  = Vec::with_capacity( member_count );
     while members.len() < member_count {             
         let member: i32 =  rand::thread_rng().gen_range( 1, set_size + 1  ) as i32;
         
         // Only add new values as members.
         let new_value  = generated.insert( member );
         if new_value  {
            members.push( member )     
         }        
    }
     
     // Determine the values which are not members.
     let mut non_member_candidates: HashSet< i32 > = HashSet::with_capacity( set_size );
     for nm in 1..set_size {
         non_member_candidates.insert( nm as i32 );
     }
     let non_members: Vec<i32> = non_member_candidates.difference( &generated ).cloned().collect();
     
     return IntegerSet { size: set_size, members:  members, non_members: non_members  };
}
pub struct IntegerSet {
    pub size: usize,
    pub members: Vec<i32>,
    pub non_members: Vec<i32>
}

pub trait SetStore {
    
    fn find(
        &self,
        value: &i32
    ) -> bool;
}
