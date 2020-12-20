#[derive(Debug)]
struct Person {
    name: &'static str,
    age: i32,
}

fn print_a(a: &Person) {
    println!("print_a: a is {:?}", a);
}

fn move_a(a: Person) {
    println!("move_a: a is {:?}", a);
}

fn main() {
    let a = Person {name: "masuda", age: 50};
    print_a(&a);
    println!("main: a is {:?}", a);

    let a = Person {name: "masuda", age: 50};
    move_a(a);
    // println!("main: a is {:?}", a);

    let a = Person {name: "masuda", age: 50};
    let x = a;
    // println!("a is {:?}", a);
    println!("x is {:?}", x);
}
