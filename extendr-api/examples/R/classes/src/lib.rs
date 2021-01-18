
use extendr_api::*;

#[derive(Debug)]
struct Person {
    pub name: String,
}

#[extendr]
impl Person {
    fn new() -> Self {
        Self { name: "".to_string() }
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }
}

struct Animal<'a> {
    pub name: &'a str
}

#[extendr]
impl<'a> Animal<'a> {
    fn new () -> Self {
        Self {name: &""}
    }

    fn set_name(&mut self, name: &'a str) {
        self.name = name;
    } 

    fn name (&self) -> &str {
        self.name
    }
} 

#[extendr]
fn aux_func() {
}

// Macro to generate exports
extendr_module! {
    mod classes;
    impl Person;
    impl Animal;
    fn aux_func;
}

