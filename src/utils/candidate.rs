
mod candidate{

    pub struct Candidate{

        name: String,
        email: String,
        votes: i32,
        disqualified: bool,
    }

    impl Candidate{

        pub fn info(&self){
            println!("Name: {}, Votes: {}, Disqualified: {}",self.name,self.votes,self.disqualified);
        }

        pub fn vote(&mut self){
            self.votes += 1;
        }

    }

}