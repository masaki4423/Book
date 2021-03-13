#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("a is {:?}", a);

    let a = [1, 2, 3, 4, 5];
    dbg!(a);

    let p = Person {
        id: 100,
        name: String::from("masuda"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    dbg!(p);

    let p = Person {
        id: 100,
        name: String::from("masuda"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    println!("p is {:?}", p);
}
