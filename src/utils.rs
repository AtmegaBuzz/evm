pub mod candidate;
pub mod voter;
pub mod admin;

use admin::Admin;
use candidate::{create_candidate,Candidate};
use voter::{create_voter,Voter};
use std::io;

pub fn run_admin<T: MarkParser+Default>() -> Option<T>{

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
                let new_candidate:Candidate = create_candidate();
                return Some(new_candidate);
                },
            2 => create_voter(),
            -1 => break,
            _ => println!("Invalid Option")
        }

        
    }
    

}

