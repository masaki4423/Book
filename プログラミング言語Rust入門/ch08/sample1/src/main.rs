struct Person{
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

// struct Person{ id: i32, name: String, age: i32, addr: String }

fn main() {
    let pa = Person {
        id: 100,
        name: String::from("masuda"),
        age: 50,
        addr: String::from("Tokyo"),
    };
}
