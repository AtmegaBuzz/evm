// #[derive(Debug)]
pub struct Candidate{
    name: String,
    email: String,
    votes: u32,
    disqualified: bool,
}

impl Candidate{

    pub fn init() -> Self{
        Candidate{
            name: String::new(),
            email: String::new(),
            votes:  0,
            disqualified: false,
        }
    }

    pub fn set_name(&mut self,name:String){
        self.name = name
    }
    pub fn set_email(&mut self,email:String){
        self.email = email
    }
    pub fn set_votes(&mut self,votes:u32){
        self.votes = votes
    }
    pub fn set_disqualified(&mut self,is_disqualified:bool){
        self.disqualified = is_disqualified
    }
    
    pub fn get_name(&self) -> &String{
        &self.name
    }
    pub fn get_email(&self) -> &String{
        &self.email
    }
    pub fn get_votes(&self) -> &u32{
        &self.votes
    }
    pub fn get_disqualified(&self) -> &bool{
        &self.disqualified
    }

    pub fn info(&self){
        println!("Name: {}, Votes: {}, Disqualified: {}",self.name,self.votes,self.disqualified);
    }

    pub fn vote(&mut self){
        self.votes += 1;
    }

}




