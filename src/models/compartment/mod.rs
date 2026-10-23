#[allow(dead_code)]
pub struct Compartment {
    pub label: String,
    pub data: Vec<u8>,
}

impl<'a> Compartment {
    pub fn new(name: &str) -> Self {
        Self {
            label: name.to_string(),
            data: Vec::new(),
        }
    }

    pub fn disp(&self) -> String {
        format!(
            "\t[label] {}:\t[secret] {}\t [",
            self.label,
            String::from_utf8_lossy(&self.data)
        )
    }
}

impl std::fmt::Debug for Compartment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Label: {}\nData: {}",
            self.label,
            String::from_utf8(self.data.clone()).unwrap()
        )
    }
}
