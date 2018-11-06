// This code is editable and runnable!

extern crate rand;

mod integer_set;
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

    // Print headers.
    let domain_size: i32 = 100000;
    let search_count: usize = 100;
    println!("### Domain: 0..{}", domain_size);
    println!("### Lookups from the set: {}", search_count);
    println!("");

    // Run tests with different parameters.
    let member_count = vec!( 10, 100, 1000 );
    for mc in member_count
    {
        run_test( domain_size, mc, search_count );
    }
}

/// Runs a test.
fn run_test(
    domain_size: i32,
    member_count: usize,
    search_count: usize
)
{
    // Prepare test set
    let set = integer_set::create( domain_size as usize, member_count );

    // Prepare containers
    let sorted  = sorted_store::SortedStore::new( &set );
    let linear = linear_store::LinearStore::new( &set );
    let cuckoo = cuckoo_filter::CuckooFilterStore::new( &set ).expect( "Creating cuckoo filter failed." );
    let hash = hash_store::HashStore::new( &set );
    let ro_hash = ro_hash_store::RoHashStore::new( &set );
    let bit  = bit_store::BitStore::new( &set );

    // Generate search set
    let mut search_space: Vec<i32> = Vec::with_capacity( search_count );
    while search_space.len() < search_count {
        let value_to_search: i32 = rand::thread_rng().gen_range( 1, set.size as i32 + 1  );
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
        println!( "Search failed!" );
        println!( "* Hits from linear search: {}", hits_from_linear );
        println!( "* Hits from binary search: {}", hits_from_sorted );
        println!( "* Hits from Cuckoo search: {}", hits_from_cuckoo );
        println!( "* Hits from Array hash search: {}", hits_from_ro_hash );
        println!( "* Hits from Hash search: {}", hits_from_hash );
        println!( "* Hits from Bit search: {}", hits_from_bit );
        println!("");
    }


    println!("### Members in a set: {}", set.members.len());
    println!("");

    // Report results.
    println!( "* Linear search  took: {} ns", linear_duration.subsec_nanos()  );
    println!( "* Binary search took: {} ns", sorted_duration.subsec_nanos() );
    println!( "* Cuckoo search took: {} ns", cuckoo_duration.subsec_nanos() );
    println!( "* Array hash search took: {} ns", ro_hash_duration.subsec_nanos() );
    println!( "* Hash search took: {} ns", hash_duration.subsec_nanos() );
    println!( "* Bit search took: {} ns", bit_duration.subsec_nanos() );
    println!( "" );
    println!( "Hits: {}",  hits_from_linear );
    println!( "" );
    println!( "" );
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
