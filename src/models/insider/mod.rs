use crate::models::{ iracus::Iracus, user::User };
use crate::repository::{Repository};
pub struct Insider {
 vault: Iracus,
 pub repository: Repository,
 users_inside: Vec<User>, 
}

impl Insider {
    pub fn init() -> Self {
        Self {
            vault: Iracus::new(),
            repository: Repository::new(),
            users_inside: Vec::new(),
        }
    }

    // pub fn get_single_user(&mut self) -> &mut User {
    //     self.users_inside[0].
    // }

    pub fn add_user(&mut self, user: User) {
        self.users_inside.push(user)
    }
    
    pub fn get_demo_user() -> User {
        User::get_demo()
    }

    // temp function
    // pub fn add_pass)
    
}