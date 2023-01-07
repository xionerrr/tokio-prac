pub struct Controllers(pub i32, pub i32);

impl Controllers {
    pub fn test(name: String) -> FullName {
        {
            let surname = "Test 2".to_string();
            FullName::new(name, surname)
        }
    }
}

#[derive(Debug)]
pub struct FullName {
    pub name: String,
    pub surname: String,
}

impl FullName {
    pub fn new(name: String, surname: String) -> Self {
        Self { name, surname }
    }
}
