use crate::utils::{
    candidate::Candidate,
    voter::Voter,
    sperator_heavy,
    sperator_light
};

pub struct Db{
    candidates: Vec<Candidate>,
    voters: Vec<Voter>
    }

impl Db{

    pub fn init_db() -> Self{
        Db{
            candidates: Vec::new(),
            voters: Vec::new()
        }
    }

    pub fn append_candidate(&mut self,candidate:Candidate){
        self.candidates.push(candidate);
        println!("candidate added to db! ");
    }

    pub fn append_voter(&mut self,voter:Voter){
        self.voters.push(voter);
        println!("voter added to db! ");
    }

    pub fn list_candidates(&self){
        sperator_heavy();

        for candidate in self.candidates.iter(){

            sperator_light();  

            println!(
                "Name: {} \nEmail: {} \nVotes: {} \nDisqualified: {}",
                candidate.get_name().trim(),
                candidate.get_email().trim(),
                candidate.get_votes(),
                candidate.get_disqualified()
            );
            
            sperator_light();            
        }

        sperator_heavy();
    }


    pub fn list_voters(&self){
        sperator_heavy();

        for voter in self.voters.iter(){

            sperator_light();  

            println!(
                "Name: {} \nVoted: {}",
                voter.get_name().trim(),
                voter.get_voted(),
            );
            
            sperator_light();            
        }

        sperator_heavy();
    }

    pub fn get_candidates(&self)-> &Vec<Candidate>{
        return &self.candidates;
    }
    pub fn get_voters(&self)-> &Vec<Voter>{
        return &self.voters;
    }
    // pub fn get_candidate(&self,name:String)-> &Candidate{
    //     for candidate in self.candidates.iter(){
    //         if 
    //     }
    // }

}