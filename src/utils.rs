pub mod candidate;
pub mod voter;
pub mod admin;

use admin::Admin;
use candidate::{Candidate};
use voter::{Voter};
use std::io;

#[derive(Debug)]
pub enum Person {
    Candidate,
    Voter
}

impl Person{

    
    pub fn create_voter() -> Voter{
        let mut name = String::new();
        let mut email = String::new();

        println!("Name: ");
        io::stdin().read_line(&mut name).expect("Failed");
        println!("Email: ");
        io::stdin().read_line(&mut email).expect("Failed");


        let mut new_voter: Voter = Voter::init();

        new_voter.set_name(name);

        println!("Voter created");
        return new_voter;

    }    

    pub fn create_candidate() -> Candidate{
        let mut name = String::new();
        let mut email = String::new();
    
        println!("Name: ");
        io::stdin().read_line(&mut name).expect("Failed");
        println!("Email: ");
        io::stdin().read_line(&mut email).expect("Failed");
    
    
        let mut new_candidate: Candidate = Candidate::init();
    
        new_candidate.set_name(name);
        new_candidate.set_email(email);
    
        println!("Candidate created");
        return new_candidate;
    }
}


pub fn run_admin() -> Option<Person>{

    let admin = Admin { 
            name: String::from("admin"),
            email: String::from("admin@gmail.com"),
            password: String::from("admin"),
            active_session:false 
        };
    
    let mut username = String::from("");
    let mut password = String::from("");

    println!("Admin Username: ");
    io::stdin()
    .read_line(&mut username)
    .expect("Failed to read");

    println!("Admin Password: ");
    io::stdin()
    .read_line(&mut password)
    .expect("Failed to read");

    if !admin.verify_cred(&username.trim(), &password.trim()){
        println!("Invalid credentials");
        return None;
    }

    loop{

        println!("1 = Create new Candidate");
        println!("2 = Create new Voter");
        println!("-1 = Back");

        let mut buff = String::new();

        io::stdin()
        .read_line(&mut buff)
        .expect("Failed to read");

        let choice: i32 = buff.trim().parse::<i32>().unwrap();

        match choice {
            1 => {
                let new_candidate: Candidate = Person::create_candidate();
                return Some(Person::Candidate(new_candidate));
                },
            2 => {
                    let new_voter:Voter = Person::create_voter();
                    return Some(Person::Voter(new_voter));
                },
            -1 => break,
            _ => println!("Invalid Option")
        }

        
    }
    
    return None;

}

