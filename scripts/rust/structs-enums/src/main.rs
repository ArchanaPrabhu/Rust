use std::collections::HashMap;
use std::env::args;

#[derive(Debug)]
struct User {
    name: String,
    age: i32,
}

impl User { // what is impl?
    fn simple_string(&self) -> String {
        format!("{} - {} ", self.name, self.age)
    }

    fn grow(&mut self, h:i32) {
        self.age += 1;
    }

    fn die(self) {
        println!("Dead {}", self.simple_string());
    }
}

fn get_arg(n:usize)->Result<String, String> {
    for (i,a) in args().enumerate() {
        if (i == n) {
            return Ok(a);
        }
    }
    Err("Not enough Args".to_string())
}

fn result_option_type_enum() {
    let mut hm = HashMap::new();
    hm.insert(3, "Hello");
    hm.insert(5, "World");

    let r = hm.get(&3).unwrap(); // unwrap returns the value if there is some matching key else goes to panic state

    // hence we need to use this
    let r = hm.get(&6).unwrap_or(&"Nothing");

    let r = match hm.get(&3) {
        Some(v) => v, // what is Some(v) -  meants for some value v
        _=>"NOTHING",
    };

    println!("{}", r);

    match "3".parse::<f32>() {
        Ok(v) => println!("OK - {}", v),
        Err(e) => println!("Error - {}", e),
    }

    match get_arg(3) {
        Ok(v) => println!("OK - {}", v),
        Err(e) => println!("Error - {}", e),
    }
}


fn main() {
    let mut u = User{
        name: "Dummy".to_string(), // a trait for converting a value to a string
        age: 23,
    };

    println!("User is {}", u.simple_string());
    u.grow(1);
    println!("User is {}", u.simple_string());
    u.die();

    result_option_type_enum();
}
