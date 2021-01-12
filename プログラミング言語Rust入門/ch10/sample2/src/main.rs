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

    let mut a = Person {
        name: String::from("masuda"),
        age: 50,
    };
    println!("a is {:?}", a);
    let mut x = &mut a;
    x.age = 0;
    println!("x is {:?}", x);
    let mut y = &mut a;
    y.name = String::from("kato");
    println!("y is {:?}", y);
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