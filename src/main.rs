mod utils;
use std::io;
use utils::admin::Admin;
use utils::candidate::create_candidate;
use utils::voter::create_voter;

fn admin() -> bool{

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

    if !admin.verify_cred(&username, &password){
        println!("Invalid credentials");
        return false;
    }

    loop{

        println!("1 = Create new Candidate");
        println!("2 = Create new Voter");
        println!("-1 = Exit");

        let mut buff = String::new();

        io::stdin()
        .read_line(&mut buff)
        .expect("Failed to read");

        let choice: i32 = buff.trim().parse::<i32>().unwrap();

        match choice {
            1 => create_candidate(),
            2 => create_voter(),
            -1 => break,
            _ => println!("Invalid Option")
        }

        
    }
    
    return true;



    

}


fn run_system(){
    

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
            1 => println!("1"),
            2 => println!("2"),
            3 => println!("3"),
            -1 => break,
            _ => println!("invalid choice"),
        }

    }
}



fn main() {
    
    run_system();
    println!("System Closed!");
}
