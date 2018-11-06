
extern crate ro_scalar_set;

use integer_set;

pub struct RoHashStore<'s> {
    members: ro_scalar_set::RoScalarSet<'s, i32>

}

impl<'s> RoHashStore<'s> {

    pub fn new(
        data: & integer_set::IntegerSet
    )  -> RoHashStore{

        let members = ro_scalar_set::RoScalarSet::new( &data.members );
        return RoHashStore { members:  members };
    }
}

impl<'s> integer_set::SetStore for RoHashStore<'s> {

    fn find(
        &self,
        value: &i32
    ) -> bool {

        let found = self.members.contains( value );
        return found;
    }

}