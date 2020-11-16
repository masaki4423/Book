fn main() {
    let a = 10;
    let b = 20;
    println!("a is {}, b is {}", a, b);

    let a = 10; let b = 20; println!("a is {}, b is {}", a, b);

    let a
        = 10;
    let b
        = 20;
    println!("a is {}, b is {}",
        a, b);

    let a = 10 + 20;
    println!("a is {}", a);
    let a = { 10 + 20 };
    println!("a is {}", a);

    // let a = { 10 + 20 ;};
    // println!("a is {}", a);

    let a = add(10, 20);
    println!("a is {}", a);

    let a = 10;
    if a > 0 {
        println!("a is {}", a);
    }

    let a = 10;
    if plus(a) {
        println!("plus(a) is {}", a);
    }
}

fn add (x: i32, y:i32) -> i32 {
    x + y
}

fn plus (x: i32) -> bool {
    x > 0
}