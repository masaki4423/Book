fn main() {
    let name = "masuda tomoaki";
    let age = 30;

    let name: &str = "masuda tomoaki";
    let age: i32 = 30;

    // 浮動小数点
    let x = 100.234;
    println!("x is {}", x);
    let x: f64 = 100.234;
    println!("x is {}", x);

    // 論理値型
    let f = true;
    println!("f is {}", f);

    // 文字型
    let c = 'A';
    let c = 'あ';
    let dog = '🐶';
    let cat: char = '🐱';

    // 文字列
    let s = "Hello Rust world.";
    println!("{}", s);
    let dog = "DOG";
    let cat = "CAT";
    println!("{} and {}", dog, cat);

    let s = String::from("Hello Rust world.");
    println!("{}", s);

    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let s3 = String::from("world.");
    let s = s1 + " " + &s2 + " " + &s3;
    println!("{}", s);

    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let s3 = String::from("world.");
    let s = format!("{} {} {}", s1, s2, s3);
    println!("{}", s);

    let s1 = "Hello";
    let s2 = "Rust";
    let s3 = "world.";
    let s = format!("{} {} {}", s1, s2, s3);
    println!("{}", s);
}

fn add(x : i32, y : i32) -> i32 {
    x + y
}
