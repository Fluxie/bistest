// This code is editable and runnable!

extern crate rand;

mod integer_set;
mod ro_hash_set;
mod cuckoo_filter;
mod hash_store;
mod sorted_store;
mod linear_store;
mod bit_store;
mod ro_hash_store;

use rand::Rng;
use std::time;

use integer_set::SetStore;

fn main() {

    // Prepare test set
    let set_size: i32 = 200000;
    let member_count = 10;
    let set = integer_set::create( set_size as usize, member_count );
    
    // Prepare containers
    let sorted  = sorted_store::SortedStore::new( &set );
    let  linear = linear_store::LinearStore::new( &set );
    let cuckoo = cuckoo_filter::CuckooFilterStore::new( &set );
    let hash = hash_store::HashStore::new( &set );
    let ro_hash = ro_hash_store::RoHashStore::new( &set );
    let bit  = bit_store::BitStore::new( &set );
    
    // Generate search set
    let search_count = 100;
    let mut search_space: Vec<i32> = Vec::with_capacity( search_count );
    while search_space.len() < search_count {
        let value_to_search = rand::thread_rng().gen_range( 1, set_size+ 1  );
        search_space.push( value_to_search );
    } 
    
    // Search.    
    let cuckoo_search = search( &search_space, &cuckoo );
    let hash_search = search( &search_space, &hash );
    let sorted_search = search( &search_space, &sorted );
    let linear_search = search( &search_space, &linear );
    let bit_search = search( &search_space, &bit );
    let ro_hash_search = search( &search_space, &ro_hash );
    
    // Extract results.
    let ( hits_from_sorted, sorted_duration ) = sorted_search;
    let ( hits_from_linear, linear_duration ) = linear_search;
    let ( hits_from_hash, hash_duration ) = hash_search;
    let ( hits_from_cuckoo, cuckoo_duration ) = cuckoo_search;
    let ( hits_from_bit, bit_duration ) = bit_search;
    let ( hits_from_ro_hash, ro_hash_duration ) = ro_hash_search;
    if  hits_from_linear != hits_from_sorted || hits_from_linear != hits_from_cuckoo || hits_from_linear != hits_from_hash   || hits_from_linear != hits_from_bit  || hits_from_linear != hits_from_ro_hash {
        println!( "Search failed" );    
    }
    
    println!("Set size: {}", set.size);
    println!("Members: {}", set.members.len());
    println!("Tested values: {}", search_count);        
    
    // Report results.    
    println!( "* Linear search  took: {} ns", linear_duration.subsec_nanos()  );           
    println!( "* Binary search took: {} ns", sorted_duration.subsec_nanos() );
    println!( "* Cuckoo search took: {} ns", cuckoo_duration.subsec_nanos() );
    println!( "* Array hash search took: {} ns", ro_hash_duration.subsec_nanos() );
    println!( "* Hash search took: {} ns", hash_duration.subsec_nanos() );    
    println!( "* Bit search took: {} ns", bit_duration.subsec_nanos() );
    println!( "" );
    println!( "Hits: {}",  hits_from_linear );    
}

// Searches the given set.
fn search(
    search_space: &Vec<i32>,
    set: &integer_set::SetStore        
) -> ( i32, std::time::Duration  ) {
    
    let start = std::time::Instant::now();

    // Search for all items.
    let mut hits = 0;
    for s in search_space {
                
            let found = set.find( s );            
            if found  {
                hits += 1;
            }
    }
    
    let stop  = std::time::Instant::now();
            
    // Return the number of this.
    return ( hits, stop.duration_since( start  ) );
}
