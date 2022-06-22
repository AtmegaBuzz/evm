
pub struct DB{
    candidates: Vec<Candidate>,
    voter: Vec<Voter>
}

impl {

    pub fn init_db() -> Self{
        DB{
            candidates: Vec::new();
        }
    }

}