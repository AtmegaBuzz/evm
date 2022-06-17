use std::io::stdin;

pub struct Voter{
    name: String,
    age: u32,
    voted: bool,
}

impl Voter{

    pub fn init() -> Self {
        Voter{
            name:String::new(),
            age:0,
            voted:false,
        }
    }

    pub fn set_name(&mut self,name:String){
        self.name = name;
    }

    pub fn set_age(&mut self,age:u32){
        self.age = age;
    }

    pub fn voted(&mut self){
        self.voted = true;
    }

    pub fn info(&self){
        println!("Name: {}, Voted: {}",self.name,self.voted);
    }

    pub fn vote(&mut self){
        self.voted = true;
    }
}


pub fn create_voter(){
    let mut name = String::new();
    let mut email = String::new();

    println!("Name: ");
    stdin().read_line(&mut name).expect("Failed");
    println!("Email: ");
    stdin().read_line(&mut email).expect("Failed");


    let mut new_candidate: Voter = Voter::init();

    new_candidate.set_name(name);

    println!("Voter created");

}