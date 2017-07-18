
use integer_set;

pub struct BitStore {    
    members: Vec<i8>     
    
}

impl BitStore {
    
    pub fn new(         
        data: & integer_set::IntegerSet
    )  -> BitStore{        
                
        let mut  members: Vec<i8> = vec![ 0; data.size / 8 + 1  ];
        for m in &data.members {
            let ( index, bit ) = as_bit( &m );            
            members[ index ] |= bit;
        }
        return BitStore { members: members };
    }
}

impl integer_set::SetStore for BitStore {    
    
    fn find(
        &self,
        value: &i32
    ) -> bool {
                
        let ( index, bit ) = as_bit( value );
        let found =  ( self.members[ index ] & bit ) != 0;
        return found; 
    }    
}


fn as_bit(
    value: &i32
)  -> ( usize, i8 ) {

    let index = ( value  / 8 ) as usize;
    let bit = 1 << ( value % 8)  as i8;
    return ( index, bit );
}
