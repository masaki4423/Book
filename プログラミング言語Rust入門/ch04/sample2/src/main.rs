fn main() {
    let s = "hello rust world.";
    println!("s is {}", s);

    let s = "hello rust world.";
    let hello = &s[0..5];
    let world = &s[11..];
    println!("hello is {}", hello);
    println!("world is {}", world);

    let s = "hello rust world.";
    let len = s.len();
    println!("s.len is {}", len);

    let mut s = String::new();
    s.push_str("hello ");
    s.push_str("rust ");
    s.push_str("world.");
    println!("s is {}", s);

    let hello = "HELLO";
    let rust = "RUST";
    let world = "WORLD.";
    let s = format!("{} {} {}", hello, rust, world);
    println!("s is {}", s);

    let s = "hello rust world.".to_string();
    println!("s is {}", s);
    let s = String::from("hello rust world.");
    println!("s is {}", s);
}
