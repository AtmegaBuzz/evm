use std::io;

use crate::utils::{admin::Admin,candidate::Candidate,voter::Voter,create_candidate,create_voter};


pub fn run_admin(){
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

    if admin.verify_cred(&username.trim(), &password.trim()){        
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
                    let new_candidate: Candidate = create_candidate();
                    },
                2 => {
                        let new_voter:Voter = create_voter();
                    },
                -1 => break,
                _ => println!("Invalid Option")
            }

            
        }
    }

    else{
        println!("Invalid credentials");
    }

}

pub fn run_system(){
    

    loop {
        println!("Hello Welcome to the EVM.");
        println!("  1 : Login Admin");
        println!("  2 : Login Candidate.");
        println!("  3 : Login Voter.");
        println!("  -1 : Exit.");

        let mut buff = String::from(""); 
        io::stdin()
            .read_line(&mut buff)
            .expect("Failed to read");

        let choice: i32 = buff.trim().parse::<i32>().unwrap();
        println!("{}",choice);

        match choice {
            1 => {
                let person = run_admin();
                },
            
            2 => println!("2"),
            3 => println!("3"),
            -1 => break,
            _ => println!("invalid choice"),
        }

    }
}


