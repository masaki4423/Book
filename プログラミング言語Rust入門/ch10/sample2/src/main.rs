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
}
