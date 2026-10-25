use crate::models::{ iracus::Iracus, user::User };

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

    // temp function
    // pub fn add_pass)
    
}