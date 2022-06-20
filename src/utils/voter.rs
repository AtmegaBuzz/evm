use std::io::stdin;

#[derive(Debug)]
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

