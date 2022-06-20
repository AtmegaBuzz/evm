use std::io::stdin;

#[derive(Debug)]
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

    pub fn info(&self){
        println!("Name: {}, Votes: {}, Disqualified: {}",self.name,self.votes,self.disqualified);
    }

    pub fn vote(&mut self){
        self.votes += 1;
    }

}


pub fn create_candidate() -> Candidate{
    let mut name = String::new();
    let mut email = String::new();

    println!("Name: ");
    stdin().read_line(&mut name).expect("Failed");
    println!("Email: ");
    stdin().read_line(&mut email).expect("Failed");


    let mut new_candidate: Candidate = Candidate::init();

    new_candidate.set_name(name);
    new_candidate.set_email(email);

    println!("Candidate created");
    return new_candidate;
}

