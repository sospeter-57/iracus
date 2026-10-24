use crate::models::compartment::{Compartment, Label};
use crate::models::division::Division;
use crate::models::iracus::Iracus;
use crate::utils::{InputSecret, get_uid};

#[derive(Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub compartment: Compartment,
     // current_lock: Lock,
}

impl User {

    // the blocks below were commented to try the user-sees-operates-on-compartents-first
//     // the following two functions will be called 
//     // over a series of either getting ore inside the vault
//     // or getting more outside of the vault, 
//     // but the logic is the sae for both ways
//     // if you open a lock, you must close the previous first
//     // idealy, this involves internally (through privae fields)
//     // getting locks and returning locks for iracus, divisions and invidual compartments
//     pub fn open_next(key: &str) -> bool {
//         User::close_behind();
//         false
//     }
// 
//     pub fn close_behind() {
//         
//     }
// 
//     // ths involves creattion of a new compartment
//     // a write operation
//     pub fn add_secret(division: Division, label: &str, data: Vec<u8>) -> Result<bool, String> {
// 
//         crate::
//         Err(String::from("failed to add secret to the vault"))
//     }

    pub fn new() -> Self {
        Self {
            id: get_uid().to_string(),
            username: User::get_username(),
            email: User::get_email(),
            compartment: Compartment::new(),
        }
    }

    // this function can either fetch the username from stdin
    // or from the repository
    fn get_username() -> String {
        String::from("")
    }

    // this too for now it' can fetchfrom stdin or resitory
    fn get_email() -> String {
        String::from("")
    }
    // fn fetch_compartment(&self,label: &str, key: &str) -> Compartment {
    //     self.compartment.new()
    // }

    // something cautious about add is that it'll remove any prevous labe or data present
    // relace it with the new data and  label
    pub fn add_secret(&mut self, input: InputSecret) -> Option<bool> {
        // first we should check if the compartment is open
        // the key should have performed that operation

        self.check_lock();
        
        let result = self.compartment.add(Label::from_string(input.label), input.secret.into_bytes());
        
        match result {
            Ok(()) => Some(true),
            Err(_error) => None     // take good care of that error, maybe nagivate it upwards
        }
    }

    pub fn retrieve_secret(&self) -> Result<String, String> {
        match self.compartment.retrieve() {
            Ok(secret) => Ok(secret),
            Err(error) => Err(error)
        }
    }

    pub fn update_secret(&mut self, label: Option<Label>, data: Vec<u8>) -> Result<(), String> {
        // check lock, if something fishy return Err
        match self.compartment.update(label, data) {
            Ok(()) => Ok(()),
            Err(error) => Err(error),
        }
    }

    pub fn delete(&mut self) -> Result<(), String> {
        if let Ok(okay) = self.compartment.delete() {
            Ok(okay)
        } else {
            Err(String::from(""))
        }
    }

    fn check_lock(&self) {}

    pub fn get_demo() -> Self {
        Self {
            id: get_uid(),
            username: "demo_user".to_owned(),
            email: "demouser@gmil.com".to_owned(),
            compartment: Compartment::new(),
        }
    }
}



// this represents any information the user sends by actions
// like when adding a secret, deleting, updating
pub struct UserRequest<'a, T> {
    user_division: Division,
    iracus: &'a Iracus,
    payload: T,
}

// a btw custom wrapper for all requests
struct Payload {
    
}