#![allow(dead_code)]

pub fn test() {
    
}

pub struct RoHashSet {
    storage: Vec<i32>
}

impl RoHashSet {

    pub fn new (
            values: &Vec<i32>,
            maximum: i32
    ) -> RoHashSet  {
            
        // Determine the number of buckets        
        let buckets: i32 = ( values.len() as f64 *0.05 ).ceil() as i32;
        println!( "Buckets: {}", buckets );
        let mut storage: Vec<i32> = vec!( 0; ( 1 + buckets + 1 + values.len() as i32  ) as usize );
    
        // Store the number of buckets.
        storage[ 0 ] = buckets;
        
        // Count the values required for each bucket.
        let mut values_in_buckets: Vec<i32> = vec!( 0; buckets as usize  );
        for v in values {            
            let bucket = v % buckets;
            values_in_buckets[ bucket as usize ] += 1;
        }
        
        // Set bucket pointers to point the end of each bucket.
        // They will be decrements one-by-one when the buckets are filled.
        let first_bucket: i32 = 1;
        let data_start: i32 = 1 + buckets + 1;
        let mut previous_bucket_end = data_start;
        let mut biggest_bucket: i32 = 0;
        for b in 0..buckets  {      
            
            // Track the biggest bucket.
            let values_in_bucket = values_in_buckets[ b as usize ]; 
            if  values_in_bucket > biggest_bucket {
                biggest_bucket = values_in_bucket;
            } 
                  
            storage[ ( b + first_bucket ) as usize  ] = ( previous_bucket_end + values_in_buckets[ b as usize ] ) as i32;
            previous_bucket_end = storage[ ( b + first_bucket ) as usize  ];
        }
        storage[ ( first_bucket + buckets ) as usize  ] = storage.len() as i32;
        println!( "Biggest bucket: {}", biggest_bucket );
        
        // Put values into buckets.
        let storage_end = first_bucket + first_bucket;
        for v in values {
            
            // Determine bucket for the value.
            let bucket_id = v % buckets;
            
            // Make room for the new value.
            storage[ ( bucket_id + first_bucket )  as usize ] -= 1; 
            let value_index = storage[ (  bucket_id + first_bucket )  as usize  ];
            storage[ value_index as usize  ] = *v;
        }
        
        // Sort each bucket.
        for b in 0..buckets  {
        
            // Determine the indexes.
            let begin = storage[ ( b + first_bucket ) as usize ];
            let end = storage[ ( b + first_bucket + 1 ) as usize  ];
            if( end < begin ) {
                println!( "Invalid bucket: {}", b );
            }
            
            // Get a splcie for sorting
            let  ( prologue, remainder ) =  storage.split_at_mut( begin as usize );
            let  ( bucket,  epilogue )  = remainder.split_at_mut( ( end - begin ) as usize  );
            bucket.sort();        
        }
        
        // Ensure each bucket is sorted
        for b in 0..( buckets - 1 ) {
            
            // Determine the indexes.
            let begin = storage[ ( b + first_bucket ) as usize ];
            let end = storage[ ( b + first_bucket + 1 ) as usize ];
            let mut previous: i32 = -1;
            for v_index in begin..end {
                
                if previous > storage[ v_index as usize  ] {
                    println!( "Invalid sort" );
                }
                previous = storage[ v_index as usize ];                 
            } 
        }
        
        return RoHashSet { storage: storage };
    }
    
    pub fn contains(
        &self,
        value: &i32
    ) -> bool {
        
        // Get the bucket associated with the value.
        let bucket = self.get_bucket( value );
        
        // Locate the value.
        match bucket.binary_search( value ) {
            Ok( _ ) => return true,
            Err(_) => return false,
        }        
    }
    
    /// Gets the beginning and end of the bucket.
    fn get_bucket(
        &self,
        value: &i32
    ) -> &[i32] {
        
        // Determine the bucket.
        let buckets = self.storage[ 0 ];
        let bucket_id: i32 = value % buckets;
         
        let begin = self.storage[ bucket_id as usize ];
        let end = self.storage[ ( bucket_id + 1 ) as usize ];
        
        let  ( prologue, remainder ) =  self.storage.split_at( begin as usize );
        let  ( bucket, epilogue )  = remainder.split_at( ( end - begin ) as usize );
        return bucket;        
    }
}
   