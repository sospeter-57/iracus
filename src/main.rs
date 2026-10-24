// use std::env;

mod models;
use models::{insider::Insider,  user::User};
mod utils;
use std::io::{Write, stdin, stdout};
use std::env;

use utils::*;



fn main() {
    let mut insider = Insider::init();

    // let username = "sospeter".to_owned();
    // let email = "sospeterkinyanjui57@gmail.com".to_owned();
    let mut user: User = Insider::get_demo_user();

    insider.add_user(user.clone());
    
    let vars: Vec<String> = env::args().collect();

    // // i've discovered a problem the or (||) operator can't solve, both operands being true
    // if (vars.len() == 2 && vars[2] != String::from("info")) || vars.len() != 3{
    //     panic!("either urgument or parameter not supplied")
    // }

    if vars.len() == 1 {
        panic!("didn't pass any urgument to iracus")
    }
    
    // if vars.len() == 2 && vars[1] != String::from("info") {
    //     panic!("that's definitely not the info command buddy")
    // }

    // if vars.len() != 3 {
    //     panic!("either urgument or paramter not supplied")
    // }

    let command = &vars[1];

    if command == "add" {
        let mut input_secret = InputSecret::new();
        input_secret.input();
        // let default_label = Label::default();
        if let Some(_truest) = user.add_secret(input_secret) {
            println!("secret added successfully")
        } else {
            println!("error: failed to add secret")
        }
    } else if command == "retrieve" {
        // let search_label = &vars[2];
        // let search_division = &vars[3];
        // let search_division = match search_division.as_str() {
        //     "foo" => Some(Division::Foo),
        //     "bar" => Some(Division::Bar),
        //     "zar" => Some(Division::Zar),
        //     "quix" => Some(Division::Quix),
        //     _ => None
        // };
        // let search_division = search_division.unwrap();
        
        let secret = user.retrieve_secret();
        println!("here's your long lived secret: {:?}", secret);
    } else if command == "info" {
        let info = "info";
        // let info = vault.info();
        println!("{}", info)
    } else if command == "update" {
        let secret = input_something("Please enter secret to update: ");
        match user.update_secret(None, secret.into_bytes()) {
            Ok(()) => println!("secret updated successfully"),
            Err(error) => println!("error during updating: {}", error)
        }
    } else if command == "delete" {
        match user.delete() {
            Ok(()) => println!("secret deleted successfully"),
            Err(error) => println!("error during deletion: {}", error)
        }
    } else if command == "interactive" {
        let mut movex: String = String::new();
    
        loop {
            println!("  1. add secret");
            println!("  2. retrieve secret");
            println!("  3. view vault info");
            println!(" 0. quit");
            print!("Enter move: ");
    
            stdout().flush().unwrap();
            stdin().read_line(&mut movex).unwrap();
            // let movex = movex.trim();
    
            match movex.as_str() {
                "1\n" => {
                    let mut input_secret = InputSecret::new();
                    input_secret.input();
                    if let Some(_) = user.add_secret(input_secret) {
                        println!("secret added successfully");
                    } else {
                        println!("error: failed to add secret");
                    }
                    movex.clear();
                }
                "2\n" => {
                    
                    let secret = user.retrieve_secret();
                    println!("here's your long lived secret: {:?}", secret);
                    movex.clear();
                }
                "3\n" => {
                    let info = get_info();
                    println!("{}", info);
                    movex.clear();
                }
                "0\n" => {
                    movex.clear();
                    break;
                }
                _ => println!("unknown iracus command used"),
            }
        }
    } else {
        panic!("unknown bocal command used")
    }    
    
    // vault.open("password".to_owned())
    // println!("{:#?}", vault);
}

