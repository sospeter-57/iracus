// #[allow(dead_code)]
// pub struct Compartment {
//     pub label: String,
//     pub data: Vec<u8>,
// }
// 
// impl<'a> Compartment {
//     pub fn new(name: &str) -> Self {
//         Self {
//             label: name.to_string(),
//             data: Vec::new(),
//         }
//     }
//     
//     pub fn update_secret(&mut self, data: Vec<u8>) -> Result<(), String> {
//         if !self.is_valid(data.clone()) {
//             return Err(String::from("the data format is not fit to be stored by iracus, please confirm again"));
//         }
//         self.data = data;
//         Ok(())
//     }
// 
//     pub fn disp(&self) -> String {
//         format!(
//             "\t[label] {}:\t[secret] {}\t [end]",
//             self.label,
//             String::from_utf8_lossy(&self.data)
//         )
//     }
//     
// }
// 
// 
// impl Compartment {
//     pub fn is_valid(&self, data: Vec<u8>) -> bool {
//         if data.is_empty() { false; }
//         else if data.len() != 32 { false; }
//         true
//     }
// }
// 
// impl std::fmt::Debug for Compartment {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "Label: {}\nData: {}",
//             self.label,
//             String::from_utf8(self.data.clone()).unwrap()
//         )
//     }
// }



#[derive(Clone)]
#[derive(Debug)]
pub struct Compartment {
    pub label: Option<Label>,
    data: Option<Vec<u8>>,
}

impl Compartment {
    // later we should make this method return a result, just in case the key is invalid
    pub fn new() -> Self {
        // Compartment::preload_compartment();
        Self {
            label: Some(Label::new()), data: Some(Vec::new())
        }
    }

    // this method should be the one to face the repository layer
    // in terms of reading from the repo and writing to compartment memory
    // but it's meant to be called by new directly if i'm right
    fn preload_compartment(&mut self) {
        self.label = Some(Label::new());
        self.data = Some(Vec::new());
    }

    // this should update the if already loaded compartment instance
    // and the repository
    pub fn add(&mut self, label: Label, data: Vec<u8>) -> Result<(), String> {
        self.label = Some(label);
        self.data = Some(data);

        // write to database or which every persistence layer it'll be
        Ok(())
    }

    // this should return the already loaded secret, never from repository
    pub fn retrieve(&self) -> Result<String, String> {
        match String::from_utf8(self.data.clone().unwrap()) {
            Ok(secret) => Ok(secret),
            Err(error) => Err(error.to_string()),
        }
    }

    // this works the same way as add, it's here coz in future 
    // we might decide to make one user to have different secret compartments
    // it should, just one of the unfinished features.
    // in that case update will have to know the index to update which is not implemented now of courseg    
    pub fn update(&mut self, label: Option<Label>, data: Vec<u8>) -> Result<(), String> {
        self.label = Some(label.unwrap());
        self.data = Some(data);
        Ok(())
    }

    pub fn delete(&mut self) -> Result<(), String> {
        self.label = None;
        self.data = None;
        Ok(())
    }

    pub fn disp(&self) -> String {
        format!("{:?}: {:?}", self.label.clone().unwrap().as_string(), String::from_utf8(self.data.clone().unwrap()))
    }
}


#[derive(Debug)]
#[derive(Clone)]
pub struct Label {
    inner: String
}

impl Label {
    pub fn new() -> Self {
        Self {
            inner: String::new(),
        }
    }

    pub fn default() -> Self {
        Self {
            inner: "bar".to_owned()
        }
    }

    pub fn from_string(string: String) -> Self {
        Self {
            inner: string
        }
    }

    pub fn as_string(&self) -> String {
        self.inner.clone()
    }
}