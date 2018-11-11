pub struct Environment {
    vtable: HashMap<String, String>,
    outer: Option<&Environment>,
}

impl Environment {
    pub fn new() -> Environment {
        return Environment{
            vtable: HashMap<String, String>::new(),
            outer: None,
        };
    }

    pub fn get(&self, name: String) -> Option<String> {
        return self.get(name);
    }

    pub fn set(&mut self, name: String, value: String) {
        self.insert(name, value);
    }
}
