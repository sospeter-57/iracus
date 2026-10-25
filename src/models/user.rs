use crate::models::compartment::{Compartment, Label};
use crate::models::division::Division;
use crate::models::iracus::Iracus;
use crate::services::crypt::{decrypt, encrypt};
use crate::utils::{InputSecret, get_uid};

use base64::{engine::general_purpose::STANDARD, Engine};

#[derive(Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,

    // ths field acts as an additional security mechanism
    // it' makes the password not enough to encrypt or decrypt data
    // in future it's not supposed to be hardcoded, a cleverer mechanism is need
    // but it has to stay regardless
    extra_secret: String,
    pub compartment: Compartment,
    // current_lock: Lock,
}

impl User {
    pub fn new() -> Self {
        Self {
            id: get_uid().to_string(),
            username: String::new(),
            email: String::new(),
            password: String::new(),
            extra_secret: String::from(
                "20eda417591032b549dfa913840be65add1e26086bdfe70f159077d1178d45e9",
            ),
            compartment: Compartment::new(),
        }
    }

    // something cautious about add is that it'll remove any prevous label or data present
    // relace it with the new data and  label
    pub fn add_secret(&mut self, input: InputSecret) -> bool {
        let input_data = input.secret.as_bytes();
        self.check_lock();
        if self.password.is_empty() {
            return false;
        } else {
            let cipher_text = encrypt(
                self.password.as_bytes(),
                self.extra_secret.as_bytes(),
                input_data,
            );
            let encoded = STANDARD.encode(&cipher_text);
            let result = self
                .compartment
                .add(Label::from_string(input.label), encoded.into_bytes());
            if result.is_ok() {
                true
            } else {
                false
            }
        }
    }

    pub fn retrieve_secret(&self) -> Result<String, String> {
        match self.compartment.retrieve() {
            Ok(secret) => {
                let decoded = STANDARD.decode(&secret);
                if decoded.is_ok() {
                let plain_text = decrypt(
                    self.password.as_bytes(),
                    self.extra_secret.as_bytes(),
                    decoded.unwrap().as_slice()
                )
                .unwrap();
                Ok(String::from_utf8(plain_text).unwrap())
                } else {
                    Err("failed to decode secret".to_owned())
                }
            }
            Err(error) => Err(error),
        }
    }

    pub fn update_secret(&mut self, label: Option<Label>, data: Vec<u8>) -> Result<(), String> {
        // check lock, if something fishy return Err
        let cipher_text = encrypt(
            self.password.as_bytes(),
            self.extra_secret.as_bytes(),
            String::from_utf8(data).unwrap().as_bytes(),
        );
        match self.compartment.update(label, cipher_text) {
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
            password: "".to_string(),
            extra_secret: String::from(
                "9d1949dbb70157080b28b8e7c55ac4db973c456c7b63ebf42a823b7904edb52c",
            ),
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
struct Payload {}
