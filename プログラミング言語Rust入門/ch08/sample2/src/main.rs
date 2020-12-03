struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

impl Person {
    fn print(&self) {
        println!("{}: {} ({}) in {}",
            self.id, self.name, self.age, self.addr);
    }

    fn print_t(&self, private: bool) {
        if private == true {
            println!("{}: {}",
            self.id, self.name);
        } else {
            println!("{}: {} ({}) in {}",
            self.id, self.name, self.age, self.addr);
        }
    }

    fn to_str(&self) -> String {
        let s = format!("{}: {} ({}) in {}",
            self.id, self.name, self.age, self.addr);
        s
    }
}

fn main() {
    let pa = Person {
        id: 1,
        name: String::from("masuda"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    pa.print();

    pa.print_t(true);
    pa.print_t(false);

    let s = pa.to_str();
    println!("s is {}", s);
}
