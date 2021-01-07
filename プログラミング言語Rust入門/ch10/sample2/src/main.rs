#[derive(Debug)]
struct Person {
    name: String,
    age:i32,
}

fn main() {
    let x: Person;
    {
        let a = Person {
            name: String::from("masuda"),
            age:50,
        };
        // x = &a;
        x = a;
    }
    println!("x is {:?}", x);

    let a = new_person("masuda", 50);
    println!("a is {:?}", a);
}

// fn new_person(id: i32, name: String) -> &Person {
fn new_person(name: &str, age: i32) -> Person {
    let p = Person {
        name: String::from(name),
        age: age,
    };
    // &p
    p
}