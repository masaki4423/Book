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

fn add_age(a: &mut Person) {
    a.age += 1;
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

    let mut a = Person {name: "masuda", age: 50};
    println!("a is {:?}", a);
    add_age(&mut a);
    println!("a is {:?}", a);

    // let a = Person {name: "masuda", age: 50};
    // let mut x = a;
    // println!("x is {:?}", x);
    // add_age(&mut x);
    // println!("x is {:?}", x);
    // add_age(&mut a);
    // println!("a is {:?}", a);

    // let a = Person {name: "masuda", age: 50};
    let mut a = Person {name: "masuda", age: 50};
    let mut x = &mut a;
    println!("x is {:?}", x);
    x.age += 1;
    println!("x is {:?}", x);
    add_age(&mut x);
    println!("x is {:?}", x);
}
