use crate::{models::{
    division::Division,
    iracus::Iracus,
    user::User,
}, utils::get_uid};

pub struct Insider {
 vault: Iracus,
 users_inside: Vec<User>, 
}

impl Insider {
    pub fn init() -> Self {
        Self {
            vault: Iracus::new(),
            users_inside: Vec::new(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users_inside.push(user)
    }
    
    pub fn get_demo_user() -> User {
        User::get_demo()
    }
    
}