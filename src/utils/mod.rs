
pub mod admin;
pub mod candidate;
pub mod voter;
pub mod db;

use std::io::stdin;
use candidate::Candidate;
use voter::Voter;

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


pub fn create_voter() -> Voter{
    let mut name = String::new();
    let mut email = String::new();

    println!("Name: ");
    stdin().read_line(&mut name).expect("Failed");
    println!("Email: ");
    stdin().read_line(&mut email).expect("Failed");


    let mut new_voter: Voter = Voter::init();

    new_voter.set_name(name);

    println!("Voter created");
    return new_voter;

}    


pub fn sperator_heavy(){
    println!("=====================================");
}
pub fn sperator_light(){
    println!("-------------------------------------")
}