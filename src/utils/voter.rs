

mod voter{

    pub struct Voter{
        name: String,
        age: u32,
        voted: bool,
    }

    impl Voter{

        pub fn info(&self){
            println!("Name: {}, Voted: {}",self.name,self.voted);
        }

        pub fn vote(&mut self){
            self.voted = true;
        }
    }
}