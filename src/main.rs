// This code is editable and runnable!

extern crate rand;
extern crate time;

mod integer_set;
mod cuckoo_filter;
mod hash_store;
mod sorted_store;
mod linear_store;

use rand::Rng;
use time::{Duration, PreciseTime};

use integer_set::SetStore;

fn main() {

    // Prepare test set
    let set_size: i32 = 200000;
    let member_count = 1000;
    let set = integer_set::create( set_size as usize, member_count );
    
    // Prepare containers
    let sorted  = sorted_store::SortedStore::new( &set );
    let  linear = linear_store::LinearStore::new( &set );
    let cuckoo = cuckoo_filter::CuckooFilterStore::new( &set );
    let hash = hash_store::HashStore::new( &set );
    
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
    
    // Extract results.
    let ( hits_from_sorted, sorted_duration ) = sorted_search;
    let ( hits_from_linear, linear_duration ) = linear_search;
    let ( hits_from_hash, hash_duration ) = hash_search;
    let ( hits_from_cuckoo, cuckoo_duration ) = cuckoo_search;
    if  hits_from_linear != hits_from_sorted || hits_from_linear != hits_from_cuckoo || hits_from_linear != hits_from_hash  {
        println!( "Search failed" );    
    }
    
    println!("Set size is: {}", set.size);
    println!("Member count: {}", set.members.len());    
    
    // Report results.
    println!( "Hits: {}",  hits_from_linear );    
    println!( "Linear scan took: {}", linear_duration );           
    println!( "Binary search took: {}", sorted_duration );
    println!( "Hash  search took: {}", hash_duration );
    println!( "Cuckoo  search took: {}", cuckoo_duration );
}

// Searches the given set.
fn search(
    search_space: &Vec<i32>,
    set: &integer_set::SetStore        
) -> ( i32, Duration ) {
    
    let start = PreciseTime::now();

    // Search for all items.
    let mut hits = 0;
    for s in search_space {
                
            let found = set.find( s );            
            if found  {
                hits += 1;
            }
    }
    
    let stop  = PreciseTime::now();
            
    // Return the number of this.
    return ( hits, start.to( stop ) );
}
