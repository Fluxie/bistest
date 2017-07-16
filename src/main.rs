// This code is editable and runnable!

extern crate rand;
extern crate time;

mod integer_set;
mod cuckoo_filter;
mod vector_store;

use rand::Rng;
use time::{Duration, PreciseTime};

use integer_set::SetStore;

fn main() {
    // A simple integer calculator:
    // `+` or `-` means add or subtract by 1
    // `*` or `/` means multiply or divide by 2
    let set_size: i32 = 200000;
    let member_count = 100000;
    let set = integer_set::create( set_size as usize, member_count );
    
    let  vector = vector_store::VectorStore::new( &set );
    let cuckoo = cuckoo_filter::CuckooFilterStore::new( &set );
    
    // Generate search space.
    let search_count = 1000;
    let mut search_space: Vec<i32> = Vec::with_capacity( search_count );
    while search_space.len() < search_count {
        let value_to_search = rand::thread_rng().gen_range( 1, set_size+ 1  );
        search_space.push( value_to_search );
    } 
    
    // Search.    
    let cuckoo_search = search( &search_space, &cuckoo );
    let vector_search = search( &search_space, &vector );
    
    let ( hits_from_vector, vector_duration ) = vector_search;
    let ( hits_from_cuckoo, cuckoo_duration ) = cuckoo_search;
    if hits_from_vector != hits_from_cuckoo {
        println!( "Search failed" );    
    }
    println!( "Hits: {}",  hits_from_vector );
    println!( "Vector search took: {}", vector_duration );
    println!( "Cuckoo  search took: {}", cuckoo_duration );
    
    println!("Set size is: {}", set.size);
    println!("Member count: {}", set.members.len());    

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
