struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

static mut PERSON_ID: i32 = 0;

impl Person {
    fn new(name: &str, age: i32, addr: &str) -> Person {
        let id = unsafe {
            PERSON_ID += 1;
            PERSON_ID
        };
        Person {
            id: id,
            name: name.to_string(),
            age: age,
            addr: addr.to_string(),
        }
    }

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

    fn add_age(&mut self, n: i32) {
        self.age += n;
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

    let mut pa = Person {
        id: 1,
        name: String::from("masuda"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    pa.print();
    pa.add_age(1);
    pa.print();

    let mut people = Vec::<Person>::new();
    people.push(Person::new("masuda", 50, "Tokyo"));
    people.push(Person::new("kato", 30, "Osaka"));
    people.push(Person::new("yamada", -1, "unknown"));
    people.push(Person::new("sato", -1, "unknown"));
    for p in &people {
        p.print();
    }
}
