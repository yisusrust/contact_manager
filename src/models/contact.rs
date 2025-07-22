#[derive(Clone, Debug)]
pub struct Contact {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub city: String,
}

impl Contact {
    pub fn new(name: String, email: String, phone: String, city: String) -> Self {
        Self {
            name,
            email,
            phone,
            city,
        }
    }
}
