// use std::env;
mod models;
mod repository;
mod services;
use models::{insider::Insider, user::User};
mod utils;
use std::env;
use std::io::{Write, stdin, stdout};
// use services::crypt::{encrypt};
use utils::*;

use crate::models::compartment::Label;

fn main() {
    // first we login the user
    // get their password which will be used to derive the key
    let username = input_something("Enter your username: ");
    let password = input_something("Enter your password: ");

    let mut insider = Insider::init();
    let payload = insider.repository.load_single_from_db();

    // let username = "sospeter".to_owned();
    // let email = "sospeterkinyanjui57@gmail.com".to_owned();

    let mut user: User = Insider::get_demo_user();
    user.password = password;
    user.username = username;
    user.compartment.label = Some(Label::from_string(payload.0));
    user.compartment.set_data(payload.1);
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
        input_secret.input(None);
        if user.add_secret(input_secret, &insider.repository) {
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
        match user.update_secret(None, secret.into_bytes(), &insider.repository) {
            Ok(()) => println!("secret updated successfully"),
            Err(error) => println!("error during updating: {}", error),
        }
    } else if command == "delete" {
        match user.delete(&vars[2], &mut insider.repository) {
            Ok(()) => println!("secret deleted successfully"),
            Err(error) => println!("error during deletion: {}", error),
        }
    } else if command == "interactive" {
        let mut movex: String = String::new();

        loop {
            println!("  1. add secret");
            println!("  2. retrieve secret");
            println!("  3. update secret");
            println!("  4. delete secret\n");
            println!("  9. view vault info");
            println!(" 0. quit");
            print!("Enter move: ");

            stdout().flush().unwrap();
            stdin().read_line(&mut movex).unwrap();
            // let movex = movex.trim();

            match movex.as_str() {
                "1\n" => {
                    let mut input_secret = InputSecret::new();
                    input_secret.input(None);
                    if user.add_secret(input_secret, &insider.repository) {
                        println!("secret added successfully");
                    } else {
                        println!("error: failed to add secret");
                    }
                    movex.clear();
                }
                "2\n" => {
                    let secret = user.retrieve_secret();
                    match secret {
                        Ok(secret) => println!("here's your long lived secret: {:?}", secret),
                        Err(error) => println!("this error occurred: {:?}", error),
                    }

                    movex.clear();
                }
                "3\n" => {
                    let mut new_secret = InputSecret::new();
                    new_secret.input(None);
                    match user.update_secret(
                        Some(Label::from_string(new_secret.label)),
                        new_secret.secret.into_bytes(),
                        &insider.repository,
                    ) {
                        Ok(_) => println!("secrete update successfully"),
                       Err(error) => println!("error: {:?}", error) 
                    }
                }
                "4\n" => {
                    let label = input_something("Enter secret Label: ");
                    match user.delete(label.as_str(), &mut insider.repository) {
                        Ok(_) => println!("Secret was Deleted"),
                        Err(error) => println!("Error: {:?}", error),
                    }
                }
                "9\n" => {
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
