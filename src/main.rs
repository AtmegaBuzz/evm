mod utils;
use std::io;
use utils::{run_admin,Person};

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
            1 => {
                let person:Option<Person> = run_admin();
                match person{
                    Some(Person::Candidate(Candidate)) => {
                        println!("{:?}",person);
                    }
                    Some(Person::Candidate(Voter)) => {
                        println!("Voter created");
                    }
                    _ => println!("Invalid Person returned.")
                }
            },
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
